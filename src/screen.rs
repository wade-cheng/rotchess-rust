use statig::Response;
use statig::state_machine;

#[derive(Default)]
pub struct Screen;

#[state_machine(initial = "State::start()")]
impl Screen {
    #[state]
    fn start() -> Response<State> {
        Response::Super
    }
}
