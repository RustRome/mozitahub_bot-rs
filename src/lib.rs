use serde_json;
use std::collections::HashMap;

pub type Phrases = HashMap<String, String>;

pub fn load_phrases() -> Result<Phrases, String> {
    let source = include_str!("../assets/frasi.json");

    serde_json::from_str(&source).map_err(|s| s.to_string())
}

#[cfg(test)]
mod tests {

    use super::load_phrases;

    #[test]
    fn load_phrases_test() {
        let expected = "Benvenuto nel bot di Mozilla Italia.\nUtilizzandomi potrai ottenere informazioni, richiedere supporto e molto altro.\nScopri le funzioni a tua disposizione digitando /help.\n\nHo attivato in automatico le notifiche per le future novità che annunceremo 📢. Controlla lo stato della tua iscrizione digitando /avvisi, così da poterli attivare e disattivare rapidamente.\n\nRicorda di unirti al nostro gruppo utenti Telegram, raggiungici in <a href='https://t.me/joinchat/BCql3UMy26nl4qxuRecDsQ'>Mozilla Italia - HOME</a>!";
        let phrases = load_phrases().unwrap();

        assert_eq!(expected, phrases["start"]);

        assert_eq!(None, phrases.get("end"));
    }

}
