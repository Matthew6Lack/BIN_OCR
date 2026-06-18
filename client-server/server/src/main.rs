use crate::{engine::*,actions_server::*};
use server::*;

use tokio::sync::{mpsc, oneshot};

fn main() {

    let rt = tokio::runtime::Runtime::new().unwrap();

    let (tx, rx) = {
        let _guard = rt.enter();
        mpsc::channel::<EngineAction>(32)
    };

    rt.block_on(async {
        let server = EngineServer::new(rx);
        server.start();
    });

    rt.block_on(async {
        tx.send(EngineAction::UpdateViewedImage { index: 42 }).await.unwrap();
    });

    let resultat_ocr: Option<(i32, i32)> = rt.block_on(async {
        let (solve_tx, solve_rx) = oneshot::channel();
        
        tx.send(EngineAction::Solve { respond_to: solve_tx }).await.unwrap();
        
        solve_rx.await.unwrap()
    });

    if let Some((x, y)) = resultat_ocr {
        println!("Le client synchrone a reçu le résultat de run_ocr : X={}, Y={}", x, y);
    }

    let resultat_positions: Option<Vec<(usize, usize, usize, usize)>> = rt.block_on(async {
        let (find_tx, find_rx) = oneshot::channel();
        
        tx.send(EngineAction::Find { respond_to: find_tx }).await.unwrap();
        
        find_rx.await.unwrap()
    });

    if let Some(positions) = resultat_positions {
        println!("Le client synchrone a reçu les positions : {:?}", positions);
    }
}














