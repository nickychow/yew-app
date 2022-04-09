use console_error_panic_hook::set_once as set_panic_hook;
use yew_app::App;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    set_panic_hook();
    yew::start_app::<App>();
}
