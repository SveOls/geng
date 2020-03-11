use minifb::{Key, KeyRepeat, Menu, MenuHandle, Window, WindowOptions, MouseMode, MouseButton};

use std::{
    collections::{HashMap, LinkedList, VecDeque},
    error::Error,
    mem,
    time::Duration,
};

mod rect;

pub trait Draw: std::fmt::Debug {
    fn draw(&self, img: &mut [u32], width: usize, pos: (usize, usize));
    fn width(&self) -> usize; 
    fn height(&self) -> usize;
    fn contains(&self, pos: (usize, usize)) -> bool;
    fn get_outline(&self) -> Box<dyn Draw>;
}

pub struct Image {
    window: Window,
    bg_color: u32,
    buff: Vec<u32>,
    change: bool,
    open: bool,
    effects: HashMap<Key, fn(&mut Image, Option<usize>)>,
    items: LinkedList<(Box<dyn Draw>, (usize, usize), usize)>,
    queue: (VecDeque<usize>, usize),
    selected: Option<usize>,
}

impl Image {
    pub fn new(
        name: &str,
        height: usize,
        width: usize,
        opts: WindowOptions,
        bg: Option<u32>,
    ) -> Result<Image, Box<dyn Error>> {
        let window = Window::new(name, width, height, opts)?;
        let bg_color = if let Some(a) = bg { a } else { 0xDDDDDD };
        let buff = vec![bg_color; width * height];
        Ok(Image {
            window,
            bg_color,
            buff,
            change: true,
            effects: HashMap::new(),
            open: true,
            items: LinkedList::new(),
            queue: (VecDeque::new(), 0),
            selected: None,
        })
    }
    pub fn is_open(&self) -> bool {
        self.window.is_open() && self.open
    }
    pub fn add_menu(&mut self, inp: &Menu) -> MenuHandle {
        self.window.add_menu(inp)
    }
    pub fn is_menu_pressed(&mut self) -> Option<usize> {
        self.window.is_menu_pressed()
    }
    pub fn remove_menu(&mut self, handle: MenuHandle) {
        self.window.remove_menu(handle);
    }
    pub fn limit_update_rate(&mut self, time: Option<std::time::Duration>) {
        self.window.limit_update_rate(time);
    }
    pub fn update(&mut self) {
        self.window.update();
    }
    pub fn update_with_buffer(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.change {
            Ok(self.update())
        } else {
            let (width, height) = self.get_size();

            self.buff = vec![self.bg_color; width * height];

            let t_buff = &mut Vec::new();
            mem::swap(&mut self.buff, t_buff);
            for (bitem, pos, id) in self.items.iter() {
                bitem.draw(t_buff, width, *pos);
            }
            if self.selected.is_some() {
                self.draw_selected(t_buff);
            }
            mem::swap(&mut self.buff, t_buff);

            self.window.update_with_buffer(&self.buff, width, height)?;
            self.change = false;

            Ok(())
        }
    }
    pub fn change_background(&mut self, col: u32) {
        self.bg_color = col;
        self.change = true;
    }
    pub fn get_keys_pressed(&self, repeat: KeyRepeat) -> Option<Vec<Key>> {
        self.window.get_keys_pressed(repeat)
    }

    pub fn get_effect(&mut self, inp: Key, val: Option<usize>) {
        if let Some(function) = self.effects.get(&inp) {
            function(self, val);
        }
    }

    pub fn add_effect(&mut self, inp: Key, fun: fn(&mut Image, Option<usize>)) {
        self.effects.insert(inp, fun);
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.window.get_size()
    }
    pub fn add_item(&mut self, inp: Box<dyn Draw>, pos: (usize, usize)) -> usize {
        self.change = true;
        if let Some(id) = self.queue.0.pop_front() {
            self.items.push_back((inp, (pos), id));
            id
        } else {
            self.queue.1 += 1;
            self.items.push_back((inp, (pos), self.queue.1));
            self.queue.1
        }
    }
    pub fn remove_item(&mut self, id: usize) -> Option<Box<dyn Draw>> {
        for (pos, (_, _, d)) in self.items.iter().enumerate() {
            if id == *d {
                self.queue.0.push_back(id);
                let temp = &mut self.items.split_off(pos);
                let (thing, _, _) = temp.pop_front().unwrap();
                self.items.append(temp);
                self.change = true;
                if Some(id) == self.selected {
                    self.selected = None;
                }
                return Some(thing);
            }
        }
        None
    }
    fn draw_selected(&mut self, buff: &mut [u32]) {
        for (item, pos, id) in self.items.iter() {
            if Some(*id) == self.selected {
                item.get_outline().draw(buff, self.get_size().0, *pos);
            }
        }
    }
    pub fn select(&mut self, id: usize) {
        self.selected = Some(id);
        self.change = true;
    }
    pub fn get_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        self.window.get_mouse_pos(mode)
    }
    pub fn get_mouse_down(&self, button: MouseButton) -> bool {
        self.window.get_mouse_down(button)
    }
    pub fn get_clicked(&self, x: usize, y: usize) -> Option<usize> {
        let mut dx;
        let mut dy;
        for (item, pos, id) in self.items.iter().rev() {
            dx = if let Some(a) = x.checked_sub(pos.0) {
                a
            } else { continue };
            dy = if let Some(a) = y.checked_sub(pos.1) {
                a
            } else { continue };
            if item.contains((dx, dy)) {
                return Some(*id);
            }
        }
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut wind = Image::new("tester", 512, 512, WindowOptions::default(), None)?;

    wind.add_effect(Key::Q, |img: &mut Image, _: Option<usize> | img.change_background(0xFF770000));
    wind.add_effect(Key::W, |img: &mut Image, _: Option<usize> | img.change_background(0xFF007700));
    wind.add_effect(Key::E, |img: &mut Image, _: Option<usize> | img.change_background(0xFF000077));
    wind.add_effect(Key::R, |img: &mut Image, _: Option<usize> | img.change_background(0xFFCCCCCC));
    wind.add_effect(Key::Escape, |img: &mut Image, _: Option<usize> | img.close());

    
    let test = rect::Rect::from_bmp("./data/tile.bmp")?;
    let test2 = rect::Rect::new(vec![vec![0xAA00FF00; 30]; 20]);
    let mut a = Vec::new();


    for j in 1..16 {
        for i in 0..16 {
            a.push(wind.add_item(Box::new(test.clone()), (32 * i, 32 * j)));
        }
    }



    wind.add_effect(Key::A, |img: &mut Image, _: Option<usize> | mem::drop(img.remove_item(1)));
    wind.add_effect(Key::S, |img: &mut Image, _: Option<usize> | mem::drop(img.remove_item(2)));
    wind.add_effect(Key::D, |img: &mut Image, _: Option<usize> | mem::drop(img.remove_item(3)));
    wind.add_effect(Key::F, |img: &mut Image, _: Option<usize> | mem::drop(img.remove_item(4)));
    wind.add_effect(Key::G, |img: &mut Image, _: Option<usize> | mem::drop(img.remove_item(5)));
    wind.add_effect(Key::Z, |img: &mut Image, _: Option<usize> | img.select(1));
    wind.add_effect(Key::X, |img: &mut Image, _: Option<usize> | img.select(2));
    wind.add_effect(Key::C, |img: &mut Image, _: Option<usize> | img.select(3));
    wind.add_effect(Key::V, |img: &mut Image, _: Option<usize> | img.select(4));
    wind.add_effect(Key::B, |img: &mut Image, _: Option<usize> | img.select(5));

    // wind.remove_item(a[0]);
    wind.selected = wind.get_clicked(15, 115);

    while wind.is_open() {
        wind.limit_update_rate(Some(Duration::from_millis(20)));
        wind.update_with_buffer()?;

        if let Some(keys) = wind.get_keys_pressed(KeyRepeat::No) {
            for t in keys {
                wind.get_effect(t, None);
            }
        }
        if wind.get_mouse_down(MouseButton::Left) {
            if let Some((x, y)) = wind.get_mouse_pos(MouseMode::Clamp) {
                if let Some(b) = wind.get_clicked(x as usize, y as usize) {
                    wind.select(b);
                }
            }
        } else if wind.get_mouse_down(MouseButton::Right) {
            if let Some((x, y)) = wind.get_mouse_pos(MouseMode::Clamp) {
                // wind.add_item(Box::new(test2.clone()), (x as usize, y as usize));
            }
        } 
    }
    Ok(())
}
