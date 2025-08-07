use crate::color;
use yew::{Html, classes, function_component, html};

#[function_component(Help)]
pub fn help() -> Html {
    html! {
        <div class={classes!(
            "flex", "gap-6", "flex-col",
            "p-6", "max-w-3xl", "mb-48",
        )}>
            <h1 class={classes!(
                "text-2xl", "sm:text-3xl", "font-medium"
            )}>
                { "Help and Frequently Asked Questions" }
            </h1>
            <div class={classes!(
                "w-full", "border-b",
                color!(border-700),
                "pb-5", "flex", "flex-col", "gap-5"
            )}>
                <div class={classes!(
                    "flex", "flex-col", "gap-3"
                )}>
                    <h2 class={classes!("text-xl", "oxanium")}>{ "How does the section \"Realtime\" work?" }</h2>
                    <div class={classes!("mx-4", color!(text-400))}>
                        <p class={classes!("mb-3")}>
                            { "While you are playing a game in League of Legends, data in JSON format will be available at " }
                            <span class={classes!("font-semibold", color!(text-300), "whitespace-nowrap")}>
                                { "https://127.0.0.1:2999/liveclientdata/allgamedata" }
                            </span>
                            { ", which is sent to my server and there I calculate your damages using the same formulas as those
                            you can see when hovering over an item with the setting " }
                            <span class={classes!("font-semibold", color!(text-300), "whitespace-nowrap")}>
                                { "[<Object-hover behavior>]" }
                            </span>
                            { " set to full." }
                        </p>
                        <ol class={classes!("list-disc", "space-y-3")}>
                            <li class={classes!("ml-6")}>
                                <span class={classes!(
                                    "font-medium", "text-white", "oxanium", "text-xl"
                                )}>
                                    { "Is this information stored anywhere?" }
                                </span>
                                <p>
                                    { "Yes. The game code assigned to that game and all the information is
                                    stored in a database so you can access it at any time by inserting either 
                                    the game code for that game or the game id" }
                                </p>
                            </li>
                            <li class={classes!("ml-6")}>
                                <span class={classes!(
                                    "font-medium", "text-white", "oxanium", "text-xl"
                                )}>
                                    { "What information does it contain?" }
                                </span>
                                <p>
                                    { "Summoner name of all the players that played that game, their items, runes, kills, deaths, assists,
                                    creep score, and name of the champion you played. No private information about your account is in there.
                                    You can see its model by " }
                                    <span class={classes!("font-semibold", color!(text-300), "whitespace-nowrap")}>
                                        { "clicking here" }
                                    </span>
                                </p>
                            </li>
                            <li class={classes!("ml-6")}>
                                <span class={classes!(
                                    "font-medium", "text-white", "oxanium", "text-xl"
                                )}>
                                    { "Can you read the data stored?" }
                                </span>
                                <p>
                                    { "It is stored as raw serialized bytes, so it is impossible to read it directly.
                                    If you want to access it, you have to insert the game id or the game code in the 
                                    \"History\" section and let program make this data readable" }
                                </p>
                            </li>
                        </ol>
                    </div>
                </div>
                <div class={classes!(
                    "flex", "flex-col", "gap-3"
                )}>
                    <h2 class={classes!("text-xl", "oxanium")}>{ "Why do I need to download the App to use the \"Realtime\" section?" }</h2>
                    <div class={classes!("mx-4", color!(text-400))}>
                        <p class={classes!("mb-3")}>
                            { "The only way to access your live game data is to have permission to access your machine's port 2999
                            during your game, and this is impossible to do with a regular browser like Microsoft Edge or Chrome" }
                        </p>
                    </div>
                </div>
                <div class={classes!(
                    "flex", "flex-col", "gap-3"
                )}>
                    <h2 class={classes!("text-xl", "oxanium")}>{ "Is TutorLoL safe to use?" }</h2>
                    <div class={classes!("mx-4", color!(text-400))}>
                        <p class={classes!("mb-3")}>
                            { "Yes. It was authorized and the first version of TutorLoL was released on July 1, 2023.
                            This is just a newer version of the same program, using different technologies and achieving 
                            massive performance improvements" }
                        </p>
                    </div>
                </div>
                <div class={classes!(
                    "flex", "flex-col", "gap-3"
                )}>
                    <h2 class={classes!("text-xl", "oxanium")}>{ "Are the calculated damages correct?" }</h2>
                    <div class={classes!("mx-4", color!(text-400))}>
                        <p class={classes!("mb-3")}>
                            { "Numbers will never be 100% accurate since I have no access to the current stats of
                            any of the champions in the enemy team, and some champions or items have very complex formulas. 
                            One notable example of this is Blade of the Ruined King, where it is possible to calculate the 
                            amount of attacks needed to kill the target, but only if it is evaluated alone. If this is mixed 
                            with other abilities, the result will be likely incorrect" }
                        </p>
                        <p class={classes!("mb-3")}>
                            { "If you are playing with champions that have very simple damage calculations (most of mages), you
                            only owns items that are easy to evaluate such as Nashor's Tooth, and enemies in the other team do not 
                            have specific scallings (Ornn, Sion, Malphite for example), expect results to be very accurate" }
                        </p>
                    </div>
                </div>
                <div class={classes!(
                    "flex", "flex-col", "gap-3"
                )}>
                    <h2 class={classes!("text-xl", "oxanium")}>{ "I don't want to see the source code when I hover over an item, how do I disable it?" }</h2>
                    <div class={classes!("mx-4", color!(text-400))}>
                        <p class={classes!("mb-3")}>
                            { "Go to Settings and set the configuration of " }
                            <span class={classes!("font-semibold", color!(text-300), "whitespace-nowrap")}>
                                { "[<Object-hover behavior>]" }
                            </span>
                            { " to " }
                            <span class={classes!("font-semibold", color!(text-300), "whitespace-nowrap")}>
                                { "None" }
                            </span>
                            { " or " }
                            <span class={classes!("font-semibold", color!(text-300), "whitespace-nowrap")}>
                                { "Partial" }
                            </span>
                        </p>
                    </div>
                </div>
                <div class={classes!(
                    "flex", "flex-col", "gap-3"
                )}>
                    <h2 class={classes!("text-xl", "oxanium")}>{ "How do I report a bug or request a feature?" }</h2>
                    <div class={classes!("mx-4", color!(text-400))}>
                        <p class={classes!("mb-3")}>
                            { "DM me on Discord or open an issue on GitHub in the applicable repository. If
                            the problem is related to mathematical calculations, go to the tutorlolv2 repository. 
                            Otherwise, go to the tutorlol_desktop_app repository" }
                        </p>
                    </div>
                </div>
                <div class={classes!(
                    "flex", "flex-col", "gap-3"
                )}>
                    <h2 class={classes!("text-xl", "oxanium")}>{ "Using it during a game can reduce my FPS?" }</h2>
                    <div class={classes!("mx-4", color!(text-400))}>
                        <p class={classes!("mb-3")}>
                            { "Obviously, since there's a new process running on your machine while you play, there's
                            an extra task for your processor to handle. However, TutorLoL is optimized to the most 
                            extreme level, processing and calculating your game data takes up 900 nano-seconds, and 
                            rendering (after first render) takes less than a millisecond. All three applications are 
                            compiled to machine code and uses the most recent technology available, so it is very 
                            unlikely to reduce your FPS" }
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
