use maud::{html, Render};

pub fn about() -> String {
    html! {
        p {
            "Made with "
            a href="https://www.getzola.org/" {
                "ZOLA"
            }
        }
        p {
            "Comments via "
            a href="https://mastodon.social" {
                "Mastodon"
            }
        }
        p {
            a href="https://www.sigstick.com/pack/Ds8XSC82a5s1iBSx4Bhi" {
                "Miku-Stickers"
            }
            " by "
            a href="https://www.sigstick.com/stickers?author=kal%20(store-KP-girl))" {
                "kal (store-KP-girl)"
            }
        }
        p {
            "Made with "
            span class="emoji" {
                "💕"
            }
            " in Germany"
        }
        a href="https://brainmade.org/" {
            img src="https://brainmade.org/white-logo.svg" alt="Brainmade";
        }
    }
    .render()
    .into_string()
}
