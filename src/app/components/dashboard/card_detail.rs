use std::{collections::HashMap, path::PathBuf, rc::Rc};

use endringer::types::{CommitInfo, DagInfo};
use gtk::prelude::*;
use petgraph::{graph::DiGraph, visit::EdgeRef};
use relm4::prelude::*;

pub struct CardDetailModel {
    drawing_area: Rc<gtk::DrawingArea>,
}

#[derive(Debug)]
pub enum CardDetailInput {
    // メインから drawer への命令
    RepoSelected(PathBuf),
    CloseRequested,
}

#[derive(Debug)]
pub enum CardDetailOutput {
    // drawer からメインへの通知（「閉じるボタンが押された」など）
    CloseRequested,
}

#[relm4::component(pub)]
impl Component for CardDetailModel {
    type Init = ();
    type Input = CardDetailInput;
    type Output = CardDetailOutput;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_width_request: 500,
            set_height_request: 600,
            // add_css_class: "background", // 背景色を確保
            inline_css: "background: #000;",

            // ヘッダー部分
            gtk::CenterBox {
                set_margin_all: 10,
                #[wrap(Some)]
                set_center_widget = &gtk::Label {
                    set_label: "メニュー",
                    // add_css_class: "title-4",
                },
                #[wrap(Some)]
                set_end_widget = &gtk::Button {
                    set_icon_name: "window-close-symbolic",
                    // add_css_class: "flat",
                    // メインウィンドウへ「閉じてほしい」と通知
                    connect_clicked => CardDetailInput::CloseRequested,
                }
            },

            gtk::Separator {},

            gtk::ScrolledWindow {
                #[name = "drawing_area"]
                gtk::DrawingArea {
                    set_content_width: 800,
                    set_content_height: 1000,
                    // 描画関数のセットアップ（下記 init / update 内で実行）
                }
            },

            // メニューリスト
            gtk::ListBox {
                add_css_class: "navigation-sidebar",
                set_vexpand: true,

                gtk::Label { set_label: "Zoom in", set_margin_all: 10 },
                gtk::Label { set_label: "Zoom out", set_margin_all: 10 },
            }
        }
    }

    fn init(
        _: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let model = CardDetailModel {
            drawing_area: Rc::from(widgets.drawing_area.clone()),
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            // 内部ロジックが必要な場合
            CardDetailInput::RepoSelected(path_buf) => {
                let dag = endringer::dag(&path_buf.as_path()).expect("failed to get dag");
                self.drawing_area.set_draw_func(move |_, cr, _w, _h| {
                    draw_graph(cr, &build_petgraph(&dag));
                });
            }
            CardDetailInput::CloseRequested => {
                sender.output(CardDetailOutput::CloseRequested).unwrap()
            }
        }
    }
}

fn build_petgraph(dag: &DagInfo) -> DiGraph<CommitInfo, ()> {
    let mut graph = DiGraph::<CommitInfo, ()>::new();
    let mut id_map = HashMap::new();

    // ノードの追加
    for (oid, info) in dag.nodes.clone() {
        let idx = graph.add_node(info);
        id_map.insert(oid, idx);
    }

    // エッジの追加 (子 -> 親)
    for (child_oid, parent_oid) in dag.edges.clone() {
        if let (Some(&child_idx), Some(&parent_idx)) =
            (id_map.get(&child_oid), id_map.get(&parent_oid))
        {
            graph.add_edge(child_idx, parent_idx, ());
        }
    }
    graph
}

fn draw_graph(cr: &cairo::Context, graph: &DiGraph<CommitInfo, ()>) {
    let node_radius = 10.0;
    let vertical_spacing = 50.0;
    let horizontal_offset = 50.0;

    // 簡易的な垂直レイアウト（実際にはトポロジカルソート等が必要）
    for (i, node_idx) in graph.node_indices().enumerate() {
        let x = horizontal_offset;
        let y = (i as f64 + 1.0) * vertical_spacing;

        // ノード（コミット）の描画
        cr.set_source_rgb(0.2, 0.5, 0.9);
        cr.arc(x, y, node_radius, 0.0, 2.0 * std::f64::consts::PI);
        cr.fill().expect("Failed to fill");

        // コミットメッセージの描画
        if let Some(info) = graph.node_weight(node_idx) {
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.move_to(x + 20.0, y + 5.0);
            cr.show_text(&info.summary).expect("Failed to show text");
        }

        // エッジ（親子関係）の描画
        cr.set_source_rgb(0.5, 0.5, 0.5);
        for edge in graph.edges(node_idx) {
            let target_idx = edge.target().index();
            let target_y = (target_idx as f64 + 1.0) * vertical_spacing;

            cr.move_to(x, y);
            cr.line_to(x, target_y);
            cr.stroke().expect("Failed to stroke");
        }
    }
}
