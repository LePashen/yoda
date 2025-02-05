use yoda::map::Map;
use yoda::layer::StaticLayer;
use yoda::render_target::RenderTarget;
use yoda::symbol::{CircleSymbol, LineSymbol, PolygonSymbol};
use winit::event_loop::{EventLoop, ControlFlow};
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};
use glow::HasContext;
use yoda::runtime::native::NativeRuntime;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let line_symbol = LineSymbol {width: 3.0, color: [0.5, 0.2, 0.0, 1.0], program: None};
    let line = vec![[0.0, 0.0, 0.0], [100.0, 100.0, 0.0], [100.0, 0.0, 0.0], [200.0, 0.0, 0.0], [200.0, 100.0, 0.0]];
    let line_layer = StaticLayer::new(line_symbol, vec![line]);

    let point_symbol = CircleSymbol { size: 20.0, color: [0.0, 0.7, 0.7, 1.0], program: None };
    let points = vec![[0.0, 0.0, 0.0], [100.0, 100.0, 0.0], [100.0, 0.0, 0.0], [0.0, 100.0, 0.0]];
    let point_layer = StaticLayer::new(point_symbol, points);

    let polygon_symbol = PolygonSymbol { fill_color: [0.0, 0.5, 0.3, 0.5], program: None};
    let polygon = vec![
        vec![
            [-150.0, -150.0, 0.0],
            [-150.0, 150.0, 0.0],
            [150.0, 150.0, 0.0],
            [150.0, -150.0, 0.0],
        ],
        vec![
            [-30.0, -30.0, 0.0],
            [30.0, -30.0, 0.0],
            [0.0, 30.0, 0.0],
        ]
    ];
    let polygon_layer = StaticLayer::new(polygon_symbol, vec![polygon]);

    let mut runtime = NativeRuntime::new(&|b| b.with_title("Simple yoda map example"));

    let mut map = runtime.map_mut();
    map.add_layer(Rc::new(RefCell::new(line_layer)));
    map.add_layer(Rc::new(RefCell::new(point_layer)));
    map.add_layer(Rc::new(RefCell::new(polygon_layer)));

    runtime.run();
}
