use crate::ui::UI;

pub struct App {
    ui: UI,
}

// impl Default for App {
//     fn default() -> Self {
//         Self {}
//     }
// }

impl App {
    pub fn new() -> Self {
        Self {
            ui: UI::new(),
        }
    }

    pub fn run(&self) -> anyhow::Result<()>{
        // run ui
        self.ui.run()?;

        Ok(())
    }
}
