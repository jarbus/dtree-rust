pub struct Renderer<'a:'b,'b > {

    pub c: graphics::context::Context,
    pub gl: &'a mut GlGraphics,
    pub g2d: &'b mut G2d<'b>,
    pub glyphs: Glyphs,
    pub view: View,
}
