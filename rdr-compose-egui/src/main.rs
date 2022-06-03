use eframe::egui;
use egui::{CentralPanel, Grid, SidePanel, RichText, TopBottomPanel};
use rdr_compose::{ComposeFile, index_to_attribute_id};
use rdr_compose::model::{ExecutableType, PythonEnvironment};
use rdr_compose_egui::python_management::get_conda_envs;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("连连看", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {
    compose_path: String,
    compose: Option<ComposeFile>,
    conda_envs: Option<Vec<String>>,
    selected_executable: Option<usize>,
    giving_up: bool,
    python_test_result: String,

    node_context: egui_nodes::Context,
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("/home/shiroki/.fonts/c/Casauce.Han.CN.Regular.Nerd.Font.ttf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(1, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.giving_up {
            self.compose = None;
            self.giving_up = false;
        }

        if let Some(select) = self.selected_executable {
            if !matches!(&self.compose,Some(ComposeFile{executables:ex,..} )if ex.len() > select) {
                self.selected_executable = None;
            }
        }

        self.exe_list(ctx);
        self.property_panel(ctx);

        CentralPanel::default().show(ctx, |ui| {
            use egui_nodes::{NodeConstructor, LinkArgs};
            ui.heading("连连看");
            if let Some(compose) = &mut self.compose {
                self.node_context.show(compose.executables.iter().enumerate().map(|(exe_index, exe)| {
                    let node = NodeConstructor::new(exe_index, Default::default())
                        .with_origin([225.0, 150.0].into())
                        .with_title(|ui| ui.label(&exe.name))
                        // .with_static_attribute(3, |ui| ui.label("Can't Connect to Me"))
                        ;
                    let node = exe.inputs.iter().enumerate().fold(node, |node, (in_index, input)| {
                        node.with_input_attribute(index_to_attribute_id(exe_index, in_index), Default::default(), move |ui| ui.label(input))
                    });
                    let node = exe.outputs.iter().enumerate().fold(node, |node, (out_index, output)| {
                        node.with_output_attribute(index_to_attribute_id(exe_index, out_index) + 1, Default::default(), move |ui| ui.label(output))
                    });
                    node
                }), compose.links.iter().enumerate().map(|(i, (start, end))| (i, *start, *end, LinkArgs::default())), ui);
                if let Some(idx) = self.node_context.link_destroyed() {
                    compose.links.remove(idx);
                }
                if let Some((start, end, _)) = self.node_context.link_created() {
                    compose.links.push((start, end))
                }
            }
        });
    }
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        let mut node_context = egui_nodes::Context::default();
        node_context.style = egui_nodes::Style { colors: egui_nodes::ColorStyle::colors_light(), ..Default::default() };
        // node_context.attribute_flag_push(egui_nodes::AttributeFlags::EnableLinkDetachWithDragClick);
        Self {
            compose_path: "/home/shiroki/code/rdr-rs/rdr-compose/test.toml".to_owned(),
            python_test_result: "<-点击左侧按钮进行测试".to_owned(),

            node_context,
            ..Self::default()
        }
    }

    fn property_panel(&mut self, ctx: &egui::Context) {
        if let (Some(selected), Some(compose)) = (&self.selected_executable, &mut self.compose) {
            SidePanel::right("executable_property").show(ctx, |ui| {
                let current = compose.executables.get_mut(*selected).unwrap();
                Grid::new("executable_property_grid")
                    .num_columns(2)
                    // .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("名称");
                        ui.text_edit_singleline(&mut current.name);
                        ui.end_row();

                        ui.label("路径");
                        ui.text_edit_singleline(&mut current.path);
                        ui.end_row();

                        ui.label("类型");
                        ui.horizontal(|ui| {
                            use ExecutableType::*;
                            use PythonEnvironment::System;
                            ui.selectable_value(&mut current.exe_type, CustomExecutable, "可执行文件");
                            if ui.selectable_label(matches!(current.exe_type,PythonScript(_)), "Python 脚本")
                                .clicked() { current.exe_type = PythonScript(System) };
                        });
                        ui.end_row();

                        if let ExecutableType::PythonScript(env) = &mut current.exe_type {
                            use PythonEnvironment::*;
                            ui.label("Python 环境类型");
                            ui.horizontal(|ui| {
                                ui.selectable_value(env, System, "系统自带");
                                if ui.selectable_label(matches!(env,Conda{..}), "Conda")
                                    .clicked() { *env = Conda { conda_path: compose.conda_path.clone(), conda_env: "".to_string() } };
                                if ui.selectable_label(matches!(env,VirtualEnv{..}), "VirtualEnv")
                                    .clicked() { *env = VirtualEnv { venv: "".to_owned() } };
                            });
                            ui.end_row();

                            match env {
                                Conda { conda_path: _, conda_env: env_path } => {
                                    ui.label("Conda 环境");
                                    ui.text_edit_singleline(env_path).context_menu(|ui| {
                                        if let Some(envs) = &self.conda_envs {
                                            ui.label("已检测到以下 Conda 环境：");
                                            ui.separator();
                                            for detected_env in envs {
                                                if ui.button(detected_env).clicked() {
                                                    *env_path = detected_env.clone();
                                                    ui.close_menu()
                                                }
                                            }
                                        } else {
                                            ui.label("未能获取 Conda 环境信息");
                                        }
                                    });
                                    ui.end_row();
                                }
                                VirtualEnv { venv } => {
                                    ui.label("VirtualEnv 环境");
                                    ui.text_edit_singleline(venv);
                                    ui.end_row();
                                }
                                System => {}
                            }

                            if ui.button("测试 Python 环境").clicked() {
                                if let Ok(py_version) = env.get_py_version() {
                                    self.python_test_result = py_version.trim().to_string();
                                    if let Ok(pkgs) = env.get_pip_packages() {
                                        for pip_pkg in ["pyzmq", "numpy", "protobuf"] {
                                            self.python_test_result.push_str(
                                                &if let Some(freeze) = pkgs.iter().find(|s| s.starts_with(pip_pkg)) {
                                                    format!("\n找到 {freeze}")
                                                } else {
                                                    format!("\n*{pip_pkg} 未安装！*")
                                                })
                                        }
                                    } else {
                                        self.python_test_result.push_str("\n*未能获取 pip 包信息！*")
                                    }
                                    for native_module in ["cv2"] {
                                        self.python_test_result.push_str(
                                            &if let Ok(version) = env.run_and_get_result(|cmd| {
                                                cmd.args(["-c", &format!("import {native_module}; print({native_module}.__version__)")])
                                            }) {
                                                format!("\n找到 {native_module}=={version}")
                                            } else {
                                                format!("\n*{native_module} 未安装！*")
                                            })
                                    }
                                } else {
                                    self.python_test_result = "版本测试失败！".to_string();
                                }
                            };
                            ui.vertical_centered(|ui| {
                                ui.label(&self.python_test_result);
                            });
                            ui.end_row();
                        }

                        ui.label("添加输入");
                        if ui.button("添加").clicked() {
                            current.inputs.push("很随便的输入".to_string());
                        };
                        ui.end_row();
                        ui.label("添加输出");
                        if ui.button("添加").clicked() {
                            current.outputs.push("某种输出".to_string());
                        };
                        ui.end_row();
                    });
            });
        }
    }

    fn exe_list(&mut self, ctx: &egui::Context) {
        TopBottomPanel::bottom("ddd")
            .resizable(true)
            .show(ctx, |ui| {
                if let Some(compose) = &mut self.compose {
                    ui.horizontal(|ui| {
                        ui.heading(format!("加载的文件版本为 {}", compose.version));
                        ui.separator();
                        if ui.button("放弃").clicked() { self.giving_up = true; }
                        if ui.button("保存").clicked() {
                            let serialized = toml::to_string::<ComposeFile>(compose).unwrap();
                            std::fs::write(&self.compose_path, serialized).unwrap();
                        };
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("已记录了 {} 个节点程序", compose.executables.len()));
                        ui.separator();
                        ui.add_enabled_ui(self.selected_executable.is_some(), |ui| {
                            if ui.button("删除").clicked() {
                                compose.executables.remove(self.selected_executable.unwrap());
                            }
                        });
                        if ui.button("添加").clicked() {
                            use rdr_compose::model::Executable;
                            compose.executables.push(Executable::default())
                        };
                    });
                    ui.separator();

                    use egui_extras::{Size, TableBuilder};

                    TableBuilder::new(ui)
                        .striped(true)
                        .column(Size::initial(80.0).at_least(60.0))
                        .column(Size::initial(80.0).at_least(60.0))
                        .column(Size::remainder().at_least(60.0))
                        .resizable(true)
                        .header(20.0, |mut header| {
                            header.col(|ui| { ui.heading("名称"); });
                            header.col(|ui| { ui.heading("类型"); });
                            header.col(|ui| { ui.heading("路径"); });
                        })
                        .body(|mut body| {
                            for (index, rec) in compose.executables.iter().enumerate() {
                                body.row(18.0, |mut row| {
                                    let is_current = self.selected_executable == Some(index);
                                    let try_high_light = |txt: &str| {
                                        if is_current {
                                            RichText::new(txt).underline()
                                        } else {
                                            RichText::new(txt)
                                        }
                                    };
                                    row.col(|ui| {
                                        if ui.button(try_high_light(&rec.name)).clicked() {
                                            self.selected_executable = Some(index)
                                        }
                                    });
                                    row.col(|ui| {
                                        use ExecutableType::*;
                                        ui.label(try_high_light(match &rec.exe_type {
                                            CustomExecutable => "可执行文件",
                                            PythonScript(_) => "Python 脚本",
                                        }));
                                    });
                                    row.col(|ui| { ui.label(try_high_light(&rec.path)); });
                                })
                            }
                        });
                } else {
                    ui.heading("没加载");
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.compose_path);
                        if ui.button("还不赶紧加载？").clicked() {
                            let compose_file: ComposeFile = toml::from_str(&std::fs::read_to_string(&self.compose_path).unwrap()).unwrap();
                            self.conda_envs = get_conda_envs(&compose_file.conda_path).map_err(|e| {
                                println!("{:#?}", e);
                                e
                            }).ok();
                            self.compose = Some(compose_file);
                        }
                    });
                }
            });
    }
}

