use crate::layout::Layout;
use db::Goal;
use dioxus::prelude::*;

struct Props {
    goals: Vec<Goal>
}

// Take a Vec<User> and create an HTML table.
pub fn users(goals: Vec<Goal>) -> String {

    // Inner function to create our rsx! component
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {    // <-- Use our layout
                title: "Users Table",
                table {
                    thead {
                        tr {
                            th { "ID" }
                            th { "Email" }
                        }
                    }
                    tbody {
                        cx.props.goals.iter().map(|goal| rsx!(
                            tr {
                                td {
                                    strong {
                                        "{goal.id}"
                                    }
                                }
                                td {
                                    "{goal.title}"
                                }
                            }
                        ))
                    }
                }
            }
        })
    }

    // Construct our component and render it to a string.
    let mut app = VirtualDom::new_with_props(
        app,
        Props {
            goals
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}