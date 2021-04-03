pub struct Renderer<'a, 'b>{

    pub c: graphics::context::Context,
    pub gl: &'b mut G2d<'a>,
    pub glyphs:&'b mut Glyphs,
    pub view:&'b View,
}

impl <'a, 'b> Renderer <'a, 'b> {
    pub fn new(
         c: graphics::context::Context,
         g: &'b mut G2d<'a>,
         glyphs: &'b mut Glyphs,
         view:&'b View,
) -> Renderer<'a, 'b> {
        Renderer {c, gl:g, glyphs, view}
    }
}
