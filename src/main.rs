use serde::Deserialize;
use std::fs::File;
use std::io::Write; // Pour utiliser .write_all()
use std::process::Command;

// 1. La structure pour lire la réponse de l'API
#[derive(Debug, Deserialize)]
struct ImageNasa {
    title: String,
    url: String, // L'URL de l'image (ex: https://.../image.jpg)
}

// 2. Le moteur asynchrone démarre ici
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Recherche de l'image du jour...");

    // A. On récupère les infos (JSON)
    // Note: J'utilise une clé démo, ça marche mais c'est limité
    let url_api = "https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY";

    let infos_image: ImageNasa = reqwest::get(url_api)
    .await?
    .json() // On transforme le texte en Struct
    .await?;

    println!("⭐ Image trouvée : {}", infos_image.title);
    println!("📥 Téléchargement en cours...");

    // B. On télécharge les octets de l'image (Binaire)
    let reponse_image = reqwest::get(&infos_image.url).await?;
    let octets_image = reponse_image.bytes().await?;

    // C. On sauvegarde sur le disque
    let nom_fichier = "wallpaper.jpg";
    let mut fichier = File::create(nom_fichier)?;
    fichier.write_all(&octets_image)?;

    println!("✅ Image sauvegardée sous '{}'", nom_fichier);

    // D. On change le fond d'écran (avec 'feh' pour Linux)
    // Si tu n'as pas 'feh', installe-le ou change la commande (ex: 'gsettings' sur Ubuntu)
    println!("🖼️ Application du fond d'écran...");

    Command::new("feh")
    .arg("--bg-scale")     // Option 1
    .arg(nom_fichier)      // Option 2 (le fichier qu'on vient de créer)
    .status()?;            // On attend que ce soit fini

    println!("✨ Terminé !");
    Ok(()) // Tout s'est bien passé (la fameuse boîte vide)
}
