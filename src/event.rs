use crate::map::Map;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use winit::event::MouseButton;

#[derive(Debug, Clone, Copy)]
pub struct ClickEvent {
    pub cursor_position: [i32; 2],
    pub button: MouseButton,
}

#[derive(Debug, Clone, Copy)]
pub struct DoubleClickEvent {
}

#[derive(Debug, Clone, Copy)]
pub struct DragEvent {
    pub dx: i32,
    pub dy: i32,
    pub button: MouseButton,
    pub curr_cursor_position: [i32; 2],
}

#[derive(Debug, Clone, Copy)]
pub struct ZoomEvent {
    pub delta: f32,
    pub cursor_position: [i32; 2],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventState {
    Continue,
    Final,
}

#[derive(Default)]
pub struct HandlerStore {
    next_id: usize,
    pub left_click: Vec<(usize, Rc<dyn Fn(ClickEvent, &mut Map) -> EventState>)>,
    pub double_click: Vec<(usize, Rc<dyn Fn(DoubleClickEvent, &mut Map) -> EventState>)>,
    pub drag: Vec<(usize, Rc<dyn Fn(DragEvent, &mut Map) -> EventState>)>,
    pub zoom: Vec<(usize, Rc<dyn Fn(ZoomEvent, &mut Map) -> EventState>)>,
}

impl HandlerStore {
    fn next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }
}

pub trait TypedHandlerStore<E: Copy> {
    fn get_store(&self) -> &Vec<(usize, Rc<dyn Fn(E, &mut Map) -> EventState>)>;
    fn get_store_mut(&mut self) -> &mut Vec<(usize, Rc<dyn Fn(E, &mut Map) -> EventState>)>;

    fn trigger_event(store: &Rc<RefCell<Self>>, event: E, map: &mut Map) {
        let mut handlers = vec![];
        for (_, handler) in store.borrow().get_store() {
            handlers.push(handler.clone());
        }

        for handler in handlers {
            let state = handler(event, map);
            if state == EventState::Final {
                break;
            }
        }
    }
}

impl TypedHandlerStore<ClickEvent> for HandlerStore {
    fn get_store(&self) -> &Vec<(usize, Rc<dyn Fn(ClickEvent, &mut Map) -> EventState>)> {
        &self.left_click
    }

    fn get_store_mut(&mut self) -> &mut Vec<(usize, Rc<dyn Fn(ClickEvent, &mut Map) -> EventState>)> {
        &mut self.left_click
    }
}

impl TypedHandlerStore<DoubleClickEvent> for HandlerStore {
    fn get_store(&self) -> &Vec<(usize, Rc<dyn Fn(DoubleClickEvent, &mut Map) -> EventState>)> {
        &self.double_click
    }

    fn get_store_mut(&mut self) -> &mut Vec<(usize, Rc<dyn Fn(DoubleClickEvent, &mut Map) -> EventState>)> {
        &mut self.double_click
    }
}

impl TypedHandlerStore<DragEvent> for HandlerStore {
    fn get_store(&self) -> &Vec<(usize, Rc<dyn Fn(DragEvent, &mut Map) -> EventState>)> {
        &self.drag
    }

    fn get_store_mut(&mut self) -> &mut Vec<(usize, Rc<dyn Fn(DragEvent, &mut Map) -> EventState>)> {
        &mut self.drag
    }
}

impl TypedHandlerStore<ZoomEvent> for HandlerStore {
    fn get_store(&self) -> &Vec<(usize, Rc<dyn Fn(ZoomEvent, &mut Map) -> EventState>)> {
        &self.zoom
    }

    fn get_store_mut(&mut self) -> &mut Vec<(usize, Rc<dyn Fn(ZoomEvent, &mut Map) -> EventState>)> {
        &mut self.zoom
    }
}

pub trait EventListener<E>
    where E: Copy,
          HandlerStore: TypedHandlerStore<E>
{
    fn handler_store(&self) -> Weak<RefCell<HandlerStore>>;

    fn on(&self, handler: Rc<dyn Fn(E, &mut Map) -> EventState>) -> usize {
        let store = self.handler_store().upgrade().unwrap();
        let id = store.borrow_mut().next_id();
        TypedHandlerStore::<E>::get_store_mut(&mut *store.borrow_mut()).push((id, handler));
        id
    }

    fn off(&self, handler_id: usize) {
        let store = self.handler_store().upgrade().unwrap();
        let position = TypedHandlerStore::<E>::get_store_mut(&mut *store.borrow_mut()).iter().position(|(id, _)| *id == handler_id);
        if let Some(index) = position {
            TypedHandlerStore::<E>::get_store_mut(&mut *store.borrow_mut()).remove(index);
        }
    }
}
