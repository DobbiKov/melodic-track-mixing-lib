use loggit::logger::set_log_level;
use loggit::Level;
use melodic_music_mixing::algorithm::melodic_sort;
use melodic_music_mixing::types::track::Track;

fn main() {
    set_log_level(Level::DEBUG);
    let tracks = vec![
        Track::from_pair("One - Akov, VEGAS.mp3", "5A"),
        Track::from_pair("Decisions   Phace   Mefjus.mp3", "6A"),
        Track::from_pair("Sovereign - Pythius.mp3", "7A"),
        Track::from_pair("BBT   Pythius.mp3", "7B"),
        Track::from_pair("Sovereign (Neonlight Remix)   Pythius.mp3", "7A"),
        Track::from_pair("Freefall - Despersion feat. 2Whales.mp3", "7A"),
        Track::from_pair("Gydra - Primitive Instinct (Original Mix).mp3", "8A"),
        Track::from_pair("Prophecy - Kanine.mp3", "9A"),
        Track::from_pair("audio.mp3", "9B"),
        Track::from_pair("Mantra   Noisia.mp3", "9B"),
        Track::from_pair("Entracte collapse - Magnetude, KOLT, Misanthr.mp3", "4A"),
        Track::from_pair("Friday - Magnetude.mp3", "4A"),
        Track::from_pair(
            "16_Synergy_Dark_Machine_Original_Mix_Original_Mix_muzzo_ru.mp3",
            "4A",
        ),
        Track::from_pair("Wipe feat. IHR (Original Mix).mp3", "4A"),
        Track::from_pair("Beat Down - Prolix, DC Breaks.mp3", "4A"),
        Track::from_pair("Hammerhead (Original Mix).mp3", "4A"),
        Track::from_pair("Krieg - Qo, The Clamps.mp3", "5A"),
        Track::from_pair("The Broken   Joanna Syze, barbarix.mp3", "12B"),
        Track::from_pair("Dark Days   Joanna Syze x Mizo.mp3", "12A"),
        Track::from_pair("Suspect [Synergy Remix]   Pythius.mp3", "3B"),
        Track::from_pair("Altar - Dimension.mp3", "4B"),
        Track::from_pair("Fire - Murdock, Doctrine.mp3", "4A"),
        Track::from_pair("Dust Devil - Mizo.mp3", "4A"),
        Track::from_pair("Stoning - Gydra.mp3", "4A"),
        Track::from_pair("Dominator - James Marvel, June Miller, MC Mot.mp3", "4A"),
        Track::from_pair("Peace - Despersion.mp3", "5A"),
        Track::from_pair("Joe Ford - Tomb Raver (Original Mix).mp3", "8B"),
        Track::from_pair("Fire   Blood (ABIS Remix)   3RDKND.mp3", "9B"),
        Track::from_pair("Asteroids   Prolix, Noisia.mp3", "9A"),
        Track::from_pair("Forgotten Myths   KOAN Sound.mp3", "9B"),
        Track::from_pair("I m For You   Magnetude.mp3", "9A"),
    ];

    let sorted_tracks = melodic_sort(&tracks, 10000);
    for track in sorted_tracks {
        println!("{} - {}", track.name(), track.key().unwrap())
    }
    println!("Hey")
}
