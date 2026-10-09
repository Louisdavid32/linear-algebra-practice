use nalgebra::DMatrix;
use std::io::{self, Write};

// ============================================================
// EXERCICE 03 — TRANSPOSÉE D'UNE MATRICE : RUST
// ============================================================
// 1) THÉORIE MATHÉMATIQUE
// Une matrice A de taille n x m devient A^T de taille m x n.
// La formule est : AT[j][i] = A[i][j].
// Exemple A = [[1,2,3],[4,5,6]] -> AT = [[1,4],[2,5],[3,6]].
// Il s'agit d'un changement de position des éléments, pas d'un
// calcul arithmétique entre les coefficients.
// Une matrice n'a PAS besoin d'être carrée pour être transposée.
//
// 2) VIE QUOTIDIENNE
// Ventes : les lignes peuvent être des magasins, les colonnes
// des produits. Transposer donne des lignes « produits » et des
// colonnes « magasins » sans modifier les quantités vendues.
//
// 3) INFORMATIQUE / DONNÉES
// Transposer fait passer d'un tableau observations x variables
// à un tableau variables x observations, utile pour organiser
// certains calculs et présentations de données.
//
// 4) INTELLIGENCE ARTIFICIELLE
// En régression linéaire, la transposée X^T intervient dans
// des expressions de gradient, notamment X^T (Xw - y).
// Elle permet d'aligner les dimensions des opérations.
//
// 5) ARCHITECTURE / GÉNIE CIVIL
// Des coordonnées 3D de n points peuvent être rangées en n x 3
// (un point par ligne) ou en 3 x n (un axe par ligne).
// La transposée change leur ORGANISATION dans un tableau,
// mais ne réalise pas une rotation physique du bâtiment.
//
// 6) ALGORITHME : parcourir chaque A[i][j] et copier la valeur
// dans AT[j][i]. On vérifie ensuite que (A^T)^T = A.
// ============================================================

fn lire_entier_positif(message: &str) -> usize {
    loop {
        print!("{message}");
        io::stdout().flush().expect("Impossible d'afficher le prompt");

        let mut texte = String::new();
        io::stdin().read_line(&mut texte).expect("Lecture impossible");
        match texte.trim().parse::<usize>() {
            Ok(n) if n > 0 => return n,
            _ => println!("Veuillez saisir un entier strictement positif."),
        }
    }
}

fn lire_matrice(nom: &str, n: usize, m: usize) -> Vec<Vec<f64>> {
    let mut matrice = Vec::with_capacity(n);
    println!("\nSaisie de {nom} ({n} x {m})");

    for i in 0..n {
        loop {
            print!("Ligne {} ({} nombres séparés par des espaces) : ", i + 1, m);
            io::stdout().flush().expect("Impossible d'afficher le prompt");

            let mut texte = String::new();
            io::stdin().read_line(&mut texte).expect("Lecture impossible");
            let valeurs: Result<Vec<f64>, _> = texte
                .split_whitespace()
                .map(str::parse::<f64>)
                .collect();

            match valeurs {
                Ok(ligne) if ligne.len() == m && ligne.iter().all(|v| v.is_finite()) => {
                    matrice.push(ligne);
                    break;
                }
                _ => println!("Entrez exactement {m} nombres finis."),
            }
        }
    }
    matrice
}

fn dimensions(a: &[Vec<f64>]) -> Result<(usize, usize), String> {
    let premiere = a.first().ok_or("La matrice est vide.")?;
    let m = premiere.len();
    if m == 0 || a.iter().any(|ligne| ligne.len() != m) {
        return Err("La matrice doit être rectangulaire et non vide.".into());
    }
    Ok((a.len(), m))
}

fn transposer(a: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    let (n, m) = dimensions(a)?;
    // A est n x m ; AT doit avoir m lignes et n colonnes.
    let mut at = vec![vec![0.0; n]; m];

    for i in 0..n {
        for j in 0..m {
            // i = ligne de A, j = colonne de A.
            // Exemple : A[0][2] passe à AT[2][0].
            at[j][i] = a[i][j];
        }
    }
    Ok(at)
}

fn afficher_matrice(nom: &str, a: &[Vec<f64>]) {
    println!("\n{nom} ({} x {}) =", a.len(), a[0].len());
    for ligne in a {
        print!("[");
        for (j, valeur) in ligne.iter().enumerate() {
            if j > 0 { print!("  "); }
            print!("{valeur}");
        }
        println!("]");
    }
}

fn main() {
    println!("=== EXERCICE 03 : TRANSPOSÉE D'UNE MATRICE ===");
    let n = lire_entier_positif("Nombre de lignes n : ");
    let m = lire_entier_positif("Nombre de colonnes m : ");
    let a = lire_matrice("A", n, m);
    let at = transposer(&a).expect("La saisie doit produire une matrice valide");

    afficher_matrice("A", &a);
    afficher_matrice("Transposée A^T", &at);

    // Vérification mathématique indépendante : (A^T)^T == A.
    let retour = transposer(&at).expect("La transposée doit être valide");
    println!("\nVérification (A^T)^T = A : {}", if retour == a { "OK" } else { "ERREUR" });

    // Vérification avec nalgebra, comme pour l'exercice précédent.
    // from_row_slice évite de confondre l'ordre des valeurs en mémoire.
    let valeurs: Vec<f64> = a.iter().flat_map(|ligne| ligne.iter().copied()).collect();
    let matrice_nalgebra = DMatrix::from_row_slice(n, m, &valeurs);
    let transposee_nalgebra = matrice_nalgebra.transpose();
    let mut identique = true;
    for i in 0..m {
        for j in 0..n {
            if transposee_nalgebra[(i, j)] != at[i][j] {
                identique = false;
            }
        }
    }
    println!("Vérification nalgebra : {}", if identique { "OK" } else { "ERREUR" });
}

#[cfg(test)]
mod tests {
    use super::transposer;

    #[test]
    fn matrice_rectangulaire_2x3() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        assert_eq!(transposer(&a).unwrap(), vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]);
    }

    #[test]
    fn matrice_carree_2x2() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        assert_eq!(transposer(&a).unwrap(), vec![vec![1.0, 3.0], vec![2.0, 4.0]]);
    }

    #[test]
    fn ligne_et_colonne() {
        assert_eq!(transposer(&[vec![1.0, 2.0, 3.0]]).unwrap(), vec![vec![1.0], vec![2.0], vec![3.0]]);
        assert_eq!(transposer(&[vec![1.0], vec![2.0]]).unwrap(), vec![vec![1.0, 2.0]]);
    }

    #[test]
    fn double_transposee_restitue_a() {
        let a = vec![vec![2.0, 4.0, 6.0], vec![8.0, 10.0, 12.0]];
        let at = transposer(&a).unwrap();
        assert_eq!(transposer(&at).unwrap(), a);
    }

    #[test]
    fn matrices_invalides_refusees() {
        assert!(transposer(&[]).is_err());
        assert!(transposer(&[vec![]]).is_err());
        assert!(transposer(&[vec![1.0, 2.0], vec![3.0]]).is_err());
    }
}
