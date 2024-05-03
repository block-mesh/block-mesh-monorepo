use block_mesh_common::cli::Commands;

#[derive(Debug)]
pub struct Gui {
    pub title: String,
    pub commands: Commands,
}

impl Gui {
    fn new(commands: &Commands) -> Self {
        Self {
            title: commands.to_string(),
            commands: commands.clone(),
        }
    }
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            title: Commands::Nothing.to_string(),
            commands: Commands::Nothing,
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.commands.to_string());
            match self.commands.clone() {
                Commands::ClientNode(ref mut cmd) => {
                    ui.horizontal(|ui| {
                        let keypair_path = ui.label("keypair_path: ");
                        ui.text_edit_singleline(&mut cmd.keypair_path)
                            .labelled_by(keypair_path.id);
                    });
                }
                Commands::ProxyMaster(_) => {
                    ui.label("ProxyMaster");
                }
                Commands::ProxyEndpoint(_) => {
                    ui.label("ProxyEndpoint");
                }
                Commands::Nothing => {
                    ui.label("Nothing");
                }
            }

            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.title)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Increment").clicked() {
            //     self.age += 1;
            // }
            ui.label(format!("Hello '{}'", self.title));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}

pub fn run_gui(commands: &Commands) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let gui = Gui::new(commands);
    let _ = eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<Gui>::new(gui)
        }),
    );
}
