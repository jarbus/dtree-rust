#[derive(Clone)]
pub struct Graph {
    pub nodes: HashMap<usize, Node>,
    pub selected: usize,
    pub max_id: usize,
}


impl Graph{

    pub fn new() -> Graph {
        let mut g: Graph = Graph { nodes: HashMap::new(), selected:0, max_id: 0};
        g.reset();
        return g

    }

    pub fn draw(&mut self, c: graphics::context::Context, gl: &mut GlGraphics){
        for key in self.nodes.keys() {     // draw each node
            self.nodes[key].draw(c,gl, self.nodes[&self.selected].pos);
        }

    }

    pub fn add_child(&mut self){

        self.max_id = self.max_id + 1;
        let x_offset = match self.nodes[&self.selected].children.len() {
            0 => 0.1,
            1 => - 0.1,
            _ => 0.0,
        };
        let child = Node::new(Shape::Circle, self.nodes[&self.selected].pos[0] + x_offset, self.nodes[&self.selected].pos[1] - 0.2, self.max_id, self.selected);
        self.nodes.insert(
            self.max_id,
            child,
            );
        match self.nodes.get_mut(&self.selected){
            Some(sel) => sel.children.push(self.max_id),
            _ => {}
        };
    }

    pub fn select(&mut self, new_selection: i8){
        let new_selection: usize = match new_selection {
            -1  => self.nodes[&self.selected].parent,
            x if x <= self.nodes[&self.selected].children.len() as i8 && x > 0 => self.nodes[&self.selected].children[(x-1) as usize],
            _ => self.selected,
            };

        // Unselect previous node, select new selection
        if let Some(prev_sel) = self.nodes.get_mut(&self.selected){
            prev_sel.color = DEFAULT_NODE_COLOR;
        }
        if let Some(new_sel)  =  self.nodes.get_mut(&new_selection){
            new_sel.color = SELECTED_NODE_COLOR;
        }
        self.selected = new_selection;
    }
    pub fn reset(&mut self){
        self.nodes.clear();
        self.nodes = HashMap::new();
        let mut root: Node = Node::new(Shape::Circle,0.5, 0.5, 0, 0);
        root.color = SELECTED_NODE_COLOR;
        self.nodes.insert(0, root);
        self.selected = 0;
        self.max_id = 0;
    }
}
