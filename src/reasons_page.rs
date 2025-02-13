use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(ReasonsPage)]
pub fn reasons_page() -> Html {
    let reasons = use_state(|| vec![
        "Attentionnée|Tu es tellement attentionnée, tu penses toujours aux autres avant toi-même.".to_string(),
        "Sourire|Ton sourire illumine mes journées, il est contagieux et réchauffe mon cœur.".to_string(),
        "Sensible|Ta sensibilité est une force, tu comprends les émotions des autres et tu sais toujours trouver les mots justes.".to_string(),
        "Courageuse|Tu es incroyablement courageuse, tu affrontes les défis avec détermination et tu ne te laisses jamais abattre.".to_string(),
        "Unique|Tu es unique en ton genre, il n'y a personne comme toi et c'est ce qui te rend si spéciale.".to_string(),
        "Optimiste|Ton optimisme est une source d'inspiration, tu vois toujours le bon côté des choses et tu sais comment me remonter le moral.".to_string(),
        "Mystérieuse|Ton côté mystérieux me fascine, j'ai toujours envie d'en savoir plus sur toi et de découvrir de nouvelles facettes de ta personnalité.".to_string(),
        "Naturelle|Tu es tellement naturelle et authentique, tu ne te caches pas derrière un masque et tu te montres telle que tu es.".to_string(),
        "Gentille|Tu prends de mes nouvelles, tu me soutiens et tu m'aides dans ce que je fais".to_string(),
        "Ouverte d'esprit|Ton ouverture d'esprit me permet de discuter de tout avec toi, sans jugement ni tabou.".to_string(),
        "Cultivée|J'aime discuter avec toi de sujets variés, tu as toujours quelque chose d'intéressant à dire.".to_string(),
        "Resilieente|Tu as affronter tellement de choses, et tu es toujours là avec le sourire.".to_string(),
        
        "Élégante|Ton élégance naturelle me séduit, tu as une allure raffinée et sophistiquée.".to_string(),
        "Équilibrée|Tu as un bon équilibre dans ta vie, tu sais comment concilier travail, loisirs et relations.".to_string(),
        "Joyeuse|Ta joie de vivre est communicative, tu sais comment illuminer mes journées.".to_string(),
        "Calme|Tu as un effet apaisant sur moi, tu sais comment me calmer.".to_string(),
        "Drole|Tu me fais rire, sans t'en rendre compte des fois".to_string(),
        "Calin|Tes calins sont d'une qualite... J'adore te tenir dans mes bras".to_string(),
    ]);
    
    let current_index = use_state(|| 0);

    let on_next = {
        let reasons = reasons.clone();
        let current_index = current_index.clone();
        Callback::from(move |_| {
            let next_index = (*current_index + 1) % reasons.len();
            current_index.set(next_index);
        })
    };

    let on_prev = {
        let reasons = reasons.clone();
        let current_index = current_index.clone();
        Callback::from(move |_| {
            let prev_index = if *current_index == 0 {
                reasons.len() - 1
            } else {
                *current_index - 1
            };
            current_index.set(prev_index);
        })
    };

    let current_reason = reasons[*current_index].clone();

    html! {
        <div class="reasons">
            <h1>{ "Salut ma Riri" }</h1>
            <h2>{ "Quelques raisons pour lesquelles je t'aime" }</h2>
            <div class="reason-container">
                <button class="fancy-button" onclick={on_prev}>
                        <span></span>
                        <span>{ "<" }</span>
                </button>
                <ReasonsCard reason={current_reason} />
                <button class="fancy-button" onclick={on_next}>
                        <span></span>
                        <span>{ ">" }</span>
                </button>
            </div>
        
            <Link<Route> to={Route::Ask}>
                <button class="fancy-button fini">
                        <span></span>
                        <span>{ "Fini de lire?" }</span>
                </button>
            </Link<Route>>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ReasonsCardProps {
    reason: String,
}

#[function_component(ReasonsCard)]
fn reasons_card(props: &ReasonsCardProps) -> Html {
    html! {
        <div class="card bg-secondary two text-white p-4 border-0">
            <hr />
            <div class="container incard">
        <h2 class="indent">{ &props.reason.split("|").next().unwrap() }</h2>
        <p class="indent">{ &props.reason.split("|").skip(1).next().unwrap() }</p>
            </div>
        </div>
    }
}
