use crate::web::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Contacts",
        html! {
            h1 { "Contacts" }
            h2 { "Denominational Director" }
            h3 { "Laura Christensen Colberg" }
            a href="mailto:lchristn12@gmail.com" { "lchristn12@gmail.com" }


            h2 { "Regional Directors" }
            h3 { "Scott Sittig" }
            i { "Eastern" }
            a href="mailto:pastorscott@newhopefree.org" { "pastorscott@newhopefree.org" }

            h3 { "Paul Stackhouse" }
            i { "Mid-East" }
            a href="mailto:paul@stackhouse.org" { "paul@stackhouse.org" }

            h3 { "Jon & Anne Bartlett" }
            i { "West Coast" }
            a href="mailto:tebsmom@yahoo.com" { "tebsmom@yahoo.com" }

            h3 { "Mark Scandrett" }
            i { "At Large" }
            a href="" { "need email" }


            h2 { "Conference Directors" }

            h3 { "Bob Swank" }
            i { "Wabash" }
            a href="mailto:rswank47591@gmail.com" { "rswank47591@gmail.com" }

            h3 { "Rebekah Distaffen" }
            i { "Genesis" }
            a href="mailto:" { "need email" }

            h3 { "Todd Stendeback" }
            i { "Gateway" }
            a href="mailto:stendy4@ghotmail.com" { "stendy4@ghotmail.com" }

            h3 { "Sabrina Boggs" }
            i { "Harvest" }
            a href="" { "need email" }

            h3 { "Matt Coakley" }
            i { "North Central" }
            a href="mailto:mattcoakley92@gmail.com" { "mattcoakley92@gmail.com" }

            h3 { "Lyle & Dawn McCoon Sr." }
            i { "Michigan" }
            a href="mailto:lmccoon@ameritech.net" { "lmccoon@ameritech.net" }

            h3 { "Bruce Kaufmann (Don Kaufmann)" }
            i { "Florida" }
            a href="mailto:brucekaufmann7@gmail.com" { "brucekaufmann7@gmail.com" }

            h3 { "Greg Fields" }
            i { "Great Plains" }
            a href="mailto:fieldsgreg@rocketmail.com" { "fieldsgreg@rocketmail.com" }

            h3 { "Nick Willis" }
            i { "Oregon" }
            a href="mailto:nwillis@georgefox.edu" { "nwillis@georgefox.edu" }

            h3 { "Glenn Hasslinger, Jr." }
            i { "Pacific Northwest" }
            a href="mailto:hassl425@comcast.net" { "hassl425@comcast.net" }

            p {
                img src="/static/resources/photos/fmc-conference-map.png" {""}
            }
        },
    )
}
