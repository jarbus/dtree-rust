#[derive(Clone)]
pub struct Graph {
    pub nodes: HashMap<usize, Node>,
    pub selected: usize,
    pub max_id: usize,
}


impl Graph{

    /// Generates and returns a new graph with a single root node of id=0
    pub fn new() -> Graph {
        let mut g: Graph = Graph { nodes: HashMap::new(), selected:0, max_id: 0};
        g.reset();
        return g

    }

    /// Renders the graph with the algorithm associated with the parameter :view:
    pub fn draw_view(&mut self, c: graphics::context::Context, gl: &mut GlGraphics, view: &View){

        match view {
            // ThreeGen displays selected node, parent, and all children
            View::ThreeGen => {
                // Draw selected node in the center
                self.nodes[&self.selected].draw(c,gl, [0.5, 0.5]);
                // Draw each child in a row from left to right
                for (i, child_id) in self.nodes[&self.selected].children.iter().enumerate() {
                    self.nodes[&child_id].draw(c,gl, [(i as f64) *0.1 + 0.3, 0.7]);
                };
                // Draw a parent if parent is not selected node
                // Should only occur for root node
                let parent = self.nodes[&self.selected].parent;
                if parent != self.selected {
                    self.nodes[&parent].draw(c, gl, [0.5, 0.2]);
                    }
            },
            View::FiveGen => {},
        }
    }

    /// Adds child to currently selected node
    pub fn add_child(&mut self){

        // create a new id for the node
        self.max_id = self.max_id + 1;
        let child = Node::new(Shape::Circle, self.max_id, self.selected);
        // Insert child with id=graph.max_id
        self.nodes.insert(self.max_id, child,);
        // Add child_id to selected node's children
        match self.nodes.get_mut(&self.selected){
            Some(sel) => sel.children.push(self.max_id),
            _ => {}
        };
    }

    /// Select node in graph
    /// :new_selection: = -1 => select parent
    /// 0 <= new_selection < # of children => select child :new_selection:
    /// else keep current selection
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
    /// Clears graph, creates and selects a new root node
    pub fn reset(&mut self){
        self.nodes.clear();
        self.nodes = HashMap::new();
        let mut root: Node = Node::new(Shape::Circle, 0, 0);
        root.color = SELECTED_NODE_COLOR;
        self.nodes.insert(0, root);
        self.selected = 0;
        self.max_id = 0;
    }
}
