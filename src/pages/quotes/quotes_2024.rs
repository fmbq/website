use crate::components::layout::layout;
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Quote List 2024",
        html! {
            h1 { "Quote List 2024" }
            h2 { "Romans & James"}
            h4 { "Total: 541 verses, 48 Quotes" }
            p{
                a.button href="/static/resources/docs/2024 Quote List Romans and James.pdf" {"pdf"}
                a.button href="/static/resources/docs/2024 Quote List Romans and James.txt" {"txt"}
            }

            h2 { "Monthly Lists" }
            div.quote-month {
                div.quote-month-header { "Month 1: Romans 1-3"}
                div.quote-month-item { "Romans 1:16"}
                div.quote-month-item { "Romans 1:17"}
                div.quote-month-item { "Romans 1:20"}
                div.quote-month-item { "Romans 2:1"}
                div.quote-month-item { "Romans 2:9-11"}
                div.quote-month-item { "Romans 2:13"}
                div.quote-month-item { "Romans 3:22-24"}
                div.quote-month-footer { "[7 Quotes/92 verses]" }
            }       

            div.quote-month {
                div.quote-month-header { "Month 2: Romans 4-6"}
                div.quote-month-item { "Romans 4:3"}
                div.quote-month-item { "Romans 4:23-24"}
                div.quote-month-item { "Romans 5:1-2"}
                div.quote-month-item { "Romans 5:8"}
                div.quote-month-item { "Romans 5:17"}
                div.quote-month-item { "Romans 6:1-2"}
                div.quote-month-item { "Romans 6:11"}
                div.quote-month-item { "Romans 6:23"}
                div.quote-month-footer { "[8 Quotes/69 verses]"}
            }

            div.quote-month {
                div.quote-month-header { "Month 3: Romans 7-9"}
                div.quote-month-item { "Romans 7:4"}
                div.quote-month-item { "Romans 8:1-2"}
                div.quote-month-item { "Romans 8:16"}
                div.quote-month-item { "Romans 8:18"}
                div.quote-month-item { "Romans 8:28"}
                div.quote-month-item { "Romans 8:38-39"}
                div.quote-month-item { "Romans 9:33"}
                div.quote-month-footer { "[7 Quotes/97 verses]"}
            }

            div.quote-month {
                div.quote-month-header { "Month 4: Romans 10-12"}
                div.quote-month-item { "Romans 10:12-13"}
                div.quote-month-item { "Romans 10:17"}
                div.quote-month-item { "Romans 11:36"}
                div.quote-month-item { "Romans 12:1"}
                div.quote-month-item { "Romans 12:2"}
                div.quote-month-item { "Romans 12:9-10"}
                div.quote-month-item { "Romans 12:12"}
                div.quote-month-footer { "[7 Quotes/78 verses]"}
            }

            div.quote-month {
                div.quote-month-header { "Month 5: Romans 13-15"}
                div.quote-month-item { "Romans 13:10"}
                div.quote-month-item { "Romans 13:12"}
                div.quote-month-item { "Romans 14:8"}
                div.quote-month-item { "Romans 15:1-2"}
                div.quote-month-item { "Romans 15:5-6"}
                div.quote-month-item { "Romans 15:13"}
                div.quote-month-footer { "[6 Quotes/70 verses]"}
            }

            div.quote-month {
                div.quote-month-header { "Month 6: Romans 16, James 1-2"}
                div.quote-month-item { "Romans 16:20"}
                div.quote-month-item { "James 1:2-3"}
                div.quote-month-item { "James 1:17"}
                div.quote-month-item { "James 1:19-20"}
                div.quote-month-item { "James 1:22"}
                div.quote-month-item { "James 2:12-13"}
                div.quote-month-item { "James 2:26"}
                div.quote-month-footer { "[7 Quotes/80 verses]"}
            }
                
            div.quote-month {
                div.quote-month-header { "Month 7: James 3-5"}
                div.quote-month-item { "James 3:13"}
                div.quote-month-item { "James 3:17"}
                div.quote-month-item { "James 4:7-8"}
                div.quote-month-item { "James 4:10"}
                div.quote-month-item { "James 5:8"}
                div.quote-month-item { "James 5:16"}
                div.quote-month-footer { "[6 Quotes/55 verses]"}
            }            
        },
    )
}
