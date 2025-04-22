use crate::web::components::{
    cards::{BigImageCard, ImageAlignment},
    layout::layout,
};
use maud::{html, Markup};

pub fn render() -> Markup {
    layout(
        "Resources",
        html! {
            h1 { "Resources" }

            h2 { "Materials" }
            p {
                    "2025-2026 Acts 1-14, Galatians, Colossians, Philemon "
                        a href="/seasons/2025/quotes" {"Quote List"}
                    br;
                    "2024-2025 Romans & James "
                        a href="/seasons/2024/quotes" {"Quote List"}
                    br;

                    a href="/material" {"Past Materials Studied"}

            }

            h2 { "Rules" }

            (BigImageCard {
                title: "Official Rules",
                image_src: "/static/resources/photos/rules-1.webp",
                alignment: ImageAlignment::Left,
                body: html! {
                    p {
                        "2021 Update Highlights: "
                        ul {
                            li {"In Rookie quizzes (after the first jump) a quizzer can ask for the question to be repeated within their 20-second answer period."}
                            li {"Divisions will include Mixed Rookie unless numbers allow for ST/YT subdivisions."}
                        }
                        a href="/rules" {"HTML"} " "
                        a href="/static/resources/docs/Rules 2021 final.pdf" {"PDF"} " "
                        a href="https://docs.google.com/document/d/1LJnDxaJz6UKr92RSTxcgJJiaHt-Jj_BM/edit" target="_blank" {"WORD"}
                    }
                },
            })

            h2 { "Scoring" }

            (BigImageCard {
                title: "Scoring Instructions",
                image_src: "/static/resources/photos/score-sheet-instructions.webp",
                alignment: ImageAlignment::Right,
                body: html! {
                    p {
                        "Propert scoring is essential during each quiz round. The score keeper should (at a minimum) record:"
                        ul{
                            li { "Points earned."}
                            li { "Prejumps - missed, jump order, ..." }
                            li { "Timeouts" }
                            li { "Substitutions" }
                            li { "Appeals - who appealed and ruling" }
                        }
                        br { "For details, see " b {"2.5 POINTS AND SCORING"} " and " b {"2.6 SCORESHEET INSTRUCTIONS"} " in the 2021 Rules"}
                        br { a href="/static/resources/docs/Score Sheet Instructions.pdf" target="_blank" {"Score Sheet Instructions"}}
                        br {a href="/static/resources/docs/Score Sheet.pdf" target="_blank" {"Printable Score Sheet"}}

                    }
                },
            })


            h2 { "Study Supplies" }
            (BigImageCard {
                title: "QuizOut Publications",
                image_src: "/static/resources/photos/laura-book-1.webp",
                alignment: ImageAlignment::Left,
                body: html! {
                    p { "Portions, practice questions, software (using Bible Quiz Shop), and other study aides from Laura Colberg.
                        Materials can be ordered using forms in "
                        a href="https://docs.google.com/spreadsheets/d/1g9F4NHcvAMSy5yTiJYhInYQpiQmy1TRv/edit?usp=drive_link&ouid=115433134049665356815&rtpof=true&sd=true" target="_blank" {"Excel"}
                        " or "
                        a href="https://drive.google.com/file/d/1gfm49iBPXMe_1QV4lSeMdtoFlp5ZTLcd/view?usp=drive_link" target="_blank" {"PDF"}
                        "."
                    }
                },
            })


            (BigImageCard {
                title: "Wilmore Computer Associates",
                image_src: "/static/resources/photos/paul-book-1.webp",
                alignment: ImageAlignment::Right,
                body: html! {
                    p {"Portions, concordances, quote lists, practice questions, and other study aides from Paul Stackhouse.
                        Materials can be ordered at "
                        a href="https://biblequizzer.square.site/" target="_blank" {"biblequizzer"}
                        "."
                    }
                },
            })


            (BigImageCard {
                title: "ACME Quiz Products",
                image_src: "/static/resources/photos/acme-book-1.webp",
                alignment: ImageAlignment::Left,
                body: html! {
                    p {"Portions, activity sheets, listening CDs, and other study aides. Materials can be ordered at "
                        a href="https://acmequiz.com" {"ACME"}
                        ". Please make sure you are ordering the NIV2011 version."
                    }
                },
            })

            (BigImageCard {
                title: "Light & Life Publishing",
                image_src: "/static/resources/photos/scorepad-1.webp",
                alignment: ImageAlignment::Right,
                body: html! {
                    p {"Free Methodist Triplicate Scoresheets can be ordered from the "
                        a href="https://freemethodistbooks.com/product/bible-quiz-scoresheet-pad-of-100" {"Light and Life Bookstore"}
                    }
                },
            })


            h2 { "Electronics" }

            (BigImageCard {
                title: "Bierdeman Box",
                image_src: "/static/resources/photos/bierdeman-box-1.webp",
                alignment: ImageAlignment::Left,
                body: html! {
                    p {"This is a small digital box less than 6\" wide x 3\" tall and 1\" thick. It is available in 2 models: the traditional "Single" where the quiz box sits between the two sets of chairs, and the  Quizmaster "Dual" model where the display box sits on the front edge of the quizmaster's table and has a readout on both sides. Contact Paul Bierdeman at "
                        a href="mailto:bierdeman.paul.w@gmail.com" {"bierdeman.paul.w@gmail.com"} "."
                    }
                },
            })

            (BigImageCard {
                title: "Kirkman Quiz Equipment",
                image_src: "/static/resources/photos/kirkman-box-1.webp",
                alignment: ImageAlignment::Right,
                body: html! {
                    p {"Free Methodist Quiz Box and seatpads (select the individual options with RCA plugs). Contact Steve Kirkman through his website at "
                        a href="http://www.quizequipment.com/information.htm" {"quizequipment.com"} "."
                    }
                },
            })


            (BigImageCard {
                title: "ACME Quiz Products",
                image_src: "/static/resources/photos/acme-pad-1.webp",
                alignment: ImageAlignment::Left,
                body: html! {
                    p {"VersaPad jump pads that work with Kirkman and Bierdeman boxes. VersaPad can be ordered at "
                        a href="https://www.acmequiz.com/index.php?l=product_detail&p=2023" {"acmequiz.com"}
                        "."
                    }
                },
            })


            (BigImageCard {
                title: "McCoon Seatpads",
                image_src: "/static/resources/photos/lyle-pad-2.webp",
                alignment: ImageAlignment::Right,
                body: html! {
                    p {"Seatpads using fabric from SEED Ministries a Free Methodist ministry helping women become self-sustaining through micro-businesses creating hand-made goods sold world-wide. Cost is $10/seat pad (as of 2023). Contact Lyle McCoon, Sr. at "
                        a href="mailto:lmccoon@ameritech.net" {"lmccoon@ameritech.net"}
                        "."
                    }
                },
            })

            h2 { "Software" }
            (BigImageCard {
                title: "FMBQ Timer",
                image_src: "/static/resources/photos/fmbq-timer.webp",
                alignment: ImageAlignment::Left,
                body: html! {
                    p {"This is an Android app for Free Methodist quizzing. Provides simple timers for jumps, prejumps, appeals, and timeouts. Go to the Play Store and search for "
                        a href="https://play.google.com/store/apps/details?id=org.fmbq.timer&hl=en_US" {"FMBQ Timer"}
                        ". The app was written by Stephen Coakley."
                    }
                },
            })

        },
    )
}
