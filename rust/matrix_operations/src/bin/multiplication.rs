use nalgebra::DMatrix;
use std::io::{self, Write};

// ============================================================
// EXERCICE 02 — MULTIPLICATION MATRICIELLE (RUST)
// ============================================================
// MATHEMATIQUES
// Si A a n lignes et m colonnes, B doit avoir m lignes.
// Si B a p colonnes, C = A * B possede n lignes et p colonnes.
// Chaque coefficient : C[i][j] = somme(A[i][k] * B[k][j])
// pour k allant de 0 a m-1. Ce n'est PAS le produit case par case.
// En general, A * B != B * A.
//
// VIE COURANTE : une matrice quantites (magasins x produits)
// multipliee par une matrice prix (produits x 1) donne un total
// monetaire par magasin (magasins x 1).
//
// INFORMATIQUE : en graphisme 2D/3D, une matrice de transformation
// multiplie un vecteur de coordonnees (rotation, mise a l'echelle).
//
// INTELLIGENCE ARTIFICIELLE : les poids W multiplient les entrees X
// d'une couche neuronale ; les biais sont ensuite AJOUTES : WX + b.
//
// ARCHITECTURE / GENIE CIVIL : une transformation matricielle peut
// convertir des coordonnees d'un plan. En analyse structurelle,
// K * u = F relie matrice de rigidite, deplacements et forces.
// Ces equations sont des modeles, pas un dimensionnement complet.
// ============================================================

fn lire_entier(message: &str, min: usize, max: Option<usize>) -> usize {
    loop {
        print!("{message}");
        io::stdout().flush().expect("Impossible d'afficher le prompt");

        let mut texte = String::new();
        io::stdin().read_line(&mut texte).expect("Lecture impossible");

        match texte.trim().parse::<usize>() {
            Ok(nombre) if nombre >= min && max.is_none_or(|borne| nombre <= borne) => {
                return nombre;
            }
            _ => println!("Entrez un entier valide (minimum : {min})."),
        }
    }
}

fn lire_matrice(nom: &str, n: usize, m: usize) -> Vec<Vec<f64>> {
    println!("\nSaisie de {nom} ({n} x {m})");
    let mut matrice = Vec::with_capacity(n);

    for i in 0..n {
        loop {
            print!("Ligne {} ({} nombres separes par des espaces) : ", i + 1, m);
            io::stdout().flush().expect("Impossible d'afficher le prompt");

            let mut texte = String::new();
            io::stdin().read_line(&mut texte).expect("Lecture impossible");

            let nombres: Result<Vec<f64>, _> = texte
                .split_whitespace()
                .map(str::parse::<f64>)
                .collect();

            match nombres {
                Ok(ligne) if ligne.len() == m && ligne.iter().all(|v| v.is_finite()) => {
                    matrice.push(ligne);
                    break;
                }
                _ => println!("Saisissez exactement {m} nombres finis."),
            }
        }
    }
    matrice
}

fn dimensions(matrice: &[Vec<f64>]) -> Result<(usize, usize), String> {
    let premiere = matrice.first().ok_or("Matrice vide")?;
    let colonnes = premiere.len();
    if colonnes == 0 || matrice.iter().any(|ligne| ligne.len() != colonnes) {
        return Err("Matrice vide ou lignes de longueurs differentes".into());
    }
    Ok((matrice.len(), colonnes))
}

fn produit_matriciel(a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    let (n, m) = dimensions(a)?;
    let (p, q) = dimensions(b)?;

    if m != p {
        return Err(format!(
            "Dimensions incompatibles : A est {n}x{m}, B est {p}x{q}. Il faut {m} = {p}."
        ));
    }

    let mut c = vec![vec![0.0; q]; n];

    // i choisit une ligne de A ; j choisit une colonne de B.
    for i in 0..n {
        for j in 0..q {
            // k parcourt la ligne A[i] ET la colonne B[:, j].
            // Exemple : C[0][0] = 1*7 + 2*9 + 3*11 = 58.
            for k in 0..m {
                c[i][j] += a[i][k] * b[k][j];
            }
            if !c[i][j].is_finite() {
                return Err("Resultat numerique non fini".into());
            }
        }
    }
    Ok(c)
}

fn verifier_avec_nalgebra(a: &[Vec<f64>], b: &[Vec<f64>], c: &[Vec<f64>]) -> bool {
    let (n, m) = dimensions(a).expect("A doit etre valide");
    let (_, q) = dimensions(b).expect("B doit etre valide");
    let valeurs_a: Vec<f64> = a.iter().flatten().copied().collect();
    let valeurs_b: Vec<f64> = b.iter().flatten().copied().collect();
    let na = DMatrix::from_row_slice(n, m, &valeurs_a);
    let nb = DMatrix::from_row_slice(m, q, &valeurs_b);
    let reference = na * nb;

    for i in 0..n {
        for j in 0..q {
            let x = c[i][j];
            let y = reference[(i, j)];
            if !x.is_finite() || !y.is_finite() ||
                (x - y).abs() > 1e-9 * (1.0 + x.abs().max(y.abs())) {
                return false;
            }
        }
    }
    true
}

fn afficher_matrice(nom: &str, matrice: &[Vec<f64>]) {
    println!("\n{nom} =");
    for ligne in matrice {
        println!("{ligne:?}");
    }
}

fn main() {
    println!("=== EXERCICE 02 : MULTIPLICATION MATRICIELLE ===");
    println!("Mode 1 : A x A (A doit etre carree)");
    println!("Mode 2 : A x B (matrices compatibles, meme rectangulaires)");
    let mode = lire_entier("Mode (1 ou 2) : ", 1, Some(2));

    let (n, m) = loop {
        let n = lire_entier("Nombre de lignes de A : ", 1, None);
        let m = lire_entier("Nombre de colonnes de A : ", 1, None);
        if mode == 1 && n != m {
            println!("Pour A x A, A doit etre carree. Recommencez.");
        } else {
            break (n, m);
        }
    };
    let a = lire_matrice("A", n, m);

    let b = if mode == 1 {
        a.clone()
    } else {
        let p = loop {
            let p = lire_entier("Nombre de lignes de B : ", 1, None);
            if p == m {
                break p;
            }
            println!("Impossible : A a {m} colonnes, B doit avoir {m} lignes.");
        };
        let q = lire_entier("Nombre de colonnes de B : ", 1, None);
        lire_matrice("B", p, q)
    };

    match produit_matriciel(&a, &b) {
        Ok(c) => {
            afficher_matrice("A", &a);
            afficher_matrice("B", &b);
            afficher_matrice("C = A x B", &c);
            println!("\nVerification nalgebra : {}", if verifier_avec_nalgebra(&a, &b, &c) { "OK" } else { "ECHEC" });
        }
        Err(erreur) => eprintln!("Erreur : {erreur}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produit_rectangulaire() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let b = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];
        let c = produit_matriciel(&a, &b).unwrap();
        assert_eq!(c, vec![vec![58.0, 64.0], vec![139.0, 154.0]]);
        assert!(verifier_avec_nalgebra(&a, &b, &c));
    }

    #[test]
    fn dimensions_incompatibles() {
        let a = vec![vec![1.0, 2.0]];
        let b = vec![vec![3.0, 4.0]];
        assert!(produit_matriciel(&a, &b).is_err());
    }

    #[test]
    fn carre_2x2() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        assert_eq!(produit_matriciel(&a, &a).unwrap(), vec![vec![7.0, 10.0], vec![15.0, 22.0]]);
    }
}
