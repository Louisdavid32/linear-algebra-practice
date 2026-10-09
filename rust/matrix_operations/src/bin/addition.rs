// ============================================================
// EXERCICE 01 — ADDITION DE MATRICES EN RUST
// ============================================================
//
// DÉFINITION :
//
// L'addition matricielle consiste à additionner
// deux matrices de mêmes dimensions, élément par élément.
//
// Formule mathématique :
//
//              C[i][j] = A[i][j] + B[i][j]
//
//
// APPLICATION 1 : INFORMATIQUE
//
// Une matrice peut représenter les valeurs des pixels
// d'une image en niveaux de gris.
//
// L'addition permet notamment d'appliquer une correction
// numérique aux valeurs des pixels.
//
// Les valeurs doivent ensuite être adaptées aux limites
// du format de l'image.
//
//
// APPLICATION 2 : INTELLIGENCE ARTIFICIELLE
//
// Dans un réseau de neurones, on rencontre :
//
//                  Z = W * X + B
//
// W : matrice des poids
// X : entrées
// B : biais
//
// L'addition permet d'ajouter les biais aux résultats
// des opérations précédentes.
//
//
// APPLICATION 3 : ARCHITECTURE ET BÂTIMENT
//
// Un ingénieur peut représenter différentes charges
// d'une structure à l'aide de tableaux numériques.
//
// Exemple simplifié en kN :
//
// A : charges permanentes
//
//     [[10, 20],
//      [15, 25]]
//
// B : charges d'exploitation
//
//     [[5, 8],
//      [6, 10]]
//
// Résultat A + B :
//
//     [[15, 28],
//      [21, 35]]
//
// Chaque valeur représente une somme de charges
// associées à une même zone.
//
// Ce n'est pas un calcul complet de dimensionnement :
// les combinaisons réglementaires restent nécessaires.
//
//
// APPLICATION 4 : VIE QUOTIDIENNE
//
// Une entreprise peut utiliser des matrices pour
// représenter ses ventes par magasin et par produit.
//
// Additionner les matrices de deux journées permet
// d'obtenir les ventes cumulées.
//
//
// OBJECTIF TECHNIQUE :
//
// Utiliser Vec<Vec<f64>> pour manipuler des matrices
// dont le nombre de lignes et de colonnes est variable.
//
// Les boucles for parcourent chaque élément.
//
// ============================================================

use std::io::{self, Write};

fn lire_entier(message: &str, min: usize, max: Option<usize>) -> usize {
    loop {
        print!("{message}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(n) if n >= min && max.is_none_or(|m| n <= m) => {
                return n;
            }
            _ => println!("Entier invalide. Réessayez."),
        }
    }
}

fn lire_matrice(nom: &str, n: usize, m: usize) -> Vec<Vec<f64>> {
    let mut matrice = Vec::new();

    println!("\nSaisie de la matrice {nom}");

    for i in 0..n {
        loop {
            print!("Ligne {} ({} valeurs) : ", i + 1, m);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let valeurs: Result<Vec<f64>, _> =
                input.split_whitespace()
                    .map(str::parse::<f64>)
                    .collect();

            match valeurs {
                Ok(ligne)
                    if ligne.len() == m
                        && ligne.iter().all(|v| v.is_finite()) =>
                {
                    matrice.push(ligne);
                    break;
                }
                _ => println!("Ligne invalide. Réessayez."),
            }
        }
    }

    matrice
}

fn addition_matrices(
    a: &[Vec<f64>],
    b: &[Vec<f64>],
) -> Vec<Vec<f64>> {
    let n = a.len();
    let m = a[0].len();

    let mut c = vec![vec![0.0; m]; n];

    for i in 0..n {

        // On parcourt chaque ligne de la matrice.

        for j in 0..m {

        // On parcourt chaque colonne.
        //
        // A[i][j] et B[i][j] désignent deux éléments
        // placés à la même position.
        //
        // Par exemple, pour un bâtiment :
        // A[i][j] peut être une charge permanente.
        // B[i][j] peut être une charge d'exploitation.
        //
        // C[i][j] contient leur somme.

            c[i][j] = a[i][j] + b[i][j];
        }
    }

    c
}

fn afficher_matrice(nom: &str, matrice: &[Vec<f64>]) {
    println!("\n{nom} =");

    for ligne in matrice {
        println!("{ligne:?}");
    }
}

fn main() {
    println!("=== ADDITION DE MATRICES ===");

    let mode = lire_entier(
        "Nombre de matrices à saisir (1 ou 2) : ",
        1,
        Some(2),
    );

    let n = lire_entier("Nombre de lignes : ", 1, None);
    let m = lire_entier("Nombre de colonnes : ", 1, None);

    let a = lire_matrice("A", n, m);

    let b = if mode == 1 {
        a.clone()
    } else {
        lire_matrice("B", n, m)
    };

    let c = addition_matrices(&a, &b);

    afficher_matrice("A", &a);
    afficher_matrice("B", &b);
    afficher_matrice("Résultat C", &c);
}