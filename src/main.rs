#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

/* ### MAIN ### */
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("NodeBase", options, Box::new(|_cc| Box::<App>::default()))
}

/* ### APP ### */
#[derive(Default)]
struct App {
    node_app: node_app::NodeApp,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // メニューバー

        // egui::TopBottomPanel::top("top").show(ctx, |ui| {
        //     egui::menu::bar(ui, |ui| {
        //         egui::widgets::global_dark_light_mode_switch(ui);
        //     });
        // });
        self.node_app.update(ctx, frame);
    }
}

mod node_app {
    use eframe::egui::{Frame,pos2, vec2, Color32, Pos2, Rect, Stroke, Ui, Vec2, Window, Area};
    use slotmap::SlotMap;

    // ノード
    const NODE_WIDTH: f32 = 75.0;
    const NODE_HEIGHT: f32 = 100.0;
    const NODE_SIZE: Vec2 = vec2(NODE_WIDTH, NODE_HEIGHT);
    const NODE_TOP_SIZE: Vec2 = vec2(NODE_WIDTH, 20.0);
    const NODE_ROUNDING: f32 = 10.0;
    const NODE_FILL_COLOR: Color32 = Color32::DARK_GRAY;
    const NODE_STROKE: Stroke = Stroke {
        width: 1.0,
        color: Color32::TRANSPARENT,
    };

    // ポート
    // const PORT_DIAMETER: f32 = 10.0;
    // const PORT_RADIUS: f32 = PORT_DIAMETER / 2.0;
    // const PORT_SIZE: Vec2 = vec2(PORT_DIAMETER, PORT_DIAMETER);

    // const PORT_FILL_COLOR: Color32 = Color32::LIGHT_GRAY;
    // const PORT_STROKE: Stroke = Stroke {
    //     width: 0.0,
    //     color: Color32::TRANSPARENT,
    // };

    slotmap::new_key_type! { struct NodeId; }

    enum NodeState {
        None,
        // Move(NodeId),
        // Connect(NodeId),
    }

    #[derive(Debug)]
    enum MouseState {
        None,
        // Relesaed,
        // PrimaryDown,
        SecondaryDown,
    }

    struct Node {
        center_pos: Pos2,
    }

    impl Node {
        fn _new(center_pos: Pos2) -> Self {
            Self { center_pos }
        }

        fn rect(&self) -> Rect {
            Rect::from_center_size(self.center_pos, NODE_SIZE)
        }

        fn show(&self, ui: &Ui) {
            ui.painter()
                .rect(self.rect(), NODE_ROUNDING, NODE_FILL_COLOR, NODE_STROKE);
        }
    }

    pub struct NodeApp {
        node_state: NodeState,
        mouse_state: MouseState,
        nodes: SlotMap<NodeId, Node>,
        node_pair: Vec<(NodeId, NodeId)>,
        visible_node_finder: bool,
        node_finder_pos: Pos2
    }

    impl Default for NodeApp {
        fn default() -> Self {
            Self {
                node_state: NodeState::None,
                mouse_state: MouseState::None,
                nodes: SlotMap::with_capacity_and_key(0),
                node_pair: Vec::new(),
                visible_node_finder: false,
                node_finder_pos: Pos2::ZERO,
            }
        }
    }

    impl eframe::App for NodeApp {
        fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
            eframe::egui::CentralPanel::default().show(ctx, |ui| {
                // マウスポインタの位置を取得
                let mouse = ui.input(|i| i.pointer.clone());
                let mouse_pos = mouse.interact_pos().unwrap_or(pos2(f32::MAX, f32::MAX));

                if mouse.secondary_clicked() {
                    // self.nodes.insert(Node::new(mouse_pos));
                    self.mouse_state = MouseState::SecondaryDown;
                    println!("Secondary_Click");
                }

                // todo: 右クリックでNode作成のUIを表示
                if let MouseState::SecondaryDown = self.mouse_state {
                    self.visible_node_finder = !self.visible_node_finder;
                    self.node_finder_pos = mouse_pos;
                    self.mouse_state = MouseState::None;
                }

                if self.visible_node_finder {
                    // ui.add_visible_ui(self.visible_node_finder, |ui| {
                    let node_finder_area = Area::new("ノードの追加").order(eframe::egui::Order::Foreground);
                    node_finder_area
                        .current_pos(self.node_finder_pos)
                        .show(ctx, |ui|{
                        // ui.add_enabled_ui(self.visible_node_finder, |ui|{
                            Frame::default().inner_margin(vec2(10.0, 10.0)).fill(Color32::from_gray(255)).show(ui, |ui| {
                            });
                        // });
                            
                    });
                }
                // });

                // todo: Node作成UI以外をクリックすると非表示

                // println!("{:?}", self.mouse_state);

                for (key, node) in &self.nodes {
                    let _rect = self.nodes[key].rect();

                    //各ノードの描画
                    node.show(ui);
                }
            });
        }
    }
}
