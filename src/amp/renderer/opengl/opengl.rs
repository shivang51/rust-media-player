
pub fn init(window: &mut glfw::Window){
    gl::load_with(|s| window.get_proc_address(s) as *const _);
}