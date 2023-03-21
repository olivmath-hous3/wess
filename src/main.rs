mod database;
mod logger;
mod wasm;

use logger::stdout_log;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    stdout_log("💽 start RocksDB data base").await?;
    stdout_log("🚿 Criar um canal de tarefas para receber as solicitações HTTP").await?;
    

    stdout_log("👷 Criar um conjunto de threads para executar as tarefas em segundo plano").await?;
    stdout_log("📡 Enviar as tarefas recebidas para o canal de tarefas dos runners").await?;
    stdout_log("🛰️ Configurar o servidor usando o framework Tide").await?;

    Ok(())
}
