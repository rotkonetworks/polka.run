// db.ts
use rexie::*;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct FileMetadata {
    file_id: u32,
    size: usize,
    timestamp: u64,
    status: String,
    total_chunks: u32,
    processed_chunks: u32,
    is_complete: bool,
}

#[derive(Serialize, Deserialize)]
struct MemoryData {
    file_id: u32,
    data: JsValue,
}

#[derive(Serialize, Deserialize)]
struct DisassemblyChunk {
    file_id: u32,
    chunk_id: u32,
    data: JsValue,
}

// build the database
async fn build_database() -> Result<Rexie> {
    let rexie = Rexie::builder("polkavm_files")
        .version(1)
        .add_object_store(
            ObjectStore::new("file_metadata")
                .key_path("file_id")
                .auto_increment(true)
                .add_index(Index::new("timestamp", "timestamp"))
        )
        .add_object_store(
            ObjectStore::new("memory_data")
                .key_path("file_id")
        )
        .add_object_store(
            ObjectStore::new("disassembly_data")
                .key_path("id") // id is a compound key: { file_id, chunk_id }
        )
        .build()
        .await?;

    Ok(rexie)
}

// add file metadata
async fn add_file_metadata(rexie: &Rexie, metadata: FileMetadata) -> Result<u32> {
    let transaction = rexie.transaction(&["file_metadata"], TransactionMode::ReadWrite)?;
    let metadata_store = transaction.store("file_metadata")?;

    let metadata_js = serde_wasm_bindgen::to_value(&metadata).unwrap();
    let metadata_id = metadata_store.add(&metadata_js, None).await?;
    transaction.done().await?;

    // return the metadata id
    Ok(num_traits::cast(metadata_id.as_f64().unwrap()).unwrap())
}

// add memory data
async fn add_memory_data(rexie: &Rexie, memory_data: MemoryData) -> Result<u32> {
    let transaction = rexie.transaction(&["memory_data"], TransactionMode::ReadWrite)?;
    let memory_store = transaction.store("memory_data")?;

    let memory_js = serde_wasm_bindgen::to_value(&memory_data).unwrap();
    memory_store.add(&memory_js, None).await?;
    transaction.done().await?;

    // return the memory id
    Ok(num_traits::cast(memory_id.as_f64().unwrap()).unwrap())
}

// add a disassembly chunk
async fn add_disassembly_chunk(rexie: &Rexie, chunk: DisassemblyChunk) -> Result<u32> {
    let transaction = rexie.transaction(&["disassembly_data"], TransactionMode::ReadWrite)?;
    let disassembly_store = transaction.store("disassembly_data")?;

    let chunk_js = serde_wasm_bindgen::to_value(&chunk).unwrap();
    disassembly_store.add(&chunk_js, None).await?;
    transaction.done().await?;

    // return the chunk id
    Ok(num_traits::cast(chunk_id.as_f64().unwrap()).unwrap())
}

// update metadata after adding a chunk
async fn update_metadata_after_chunk(rexie: &Rexie, file_id: u32) -> Result<()> {
    let transaction = rexie.transaction(&["file_metadata"], TransactionMode::ReadWrite)?;
    let metadata_store = transaction.store("file_metadata")?;

    if let Some(mut metadata) = get_file_metadata(&rexie, file_id).await? {
        metadata.processed_chunks += 1;
        if metadata.processed_chunks == metadata.total_chunks {
            metadata.is_complete = true;
        }

        let metadata_js = serde_wasm_bindgen::to_value(&metadata).unwrap();
        metadata_store.put(&metadata_js, &file_id.into()).await?;
    }

    transaction.done().await?;
    Ok(())
}

// get file metadata
async fn get_file_metadata(rexie: &Rexie, file_id: u32) -> Result<Option<FileMetadata>> {
    let transaction = rexie.transaction(&["file_metadata"], TransactionMode::ReadOnly)?;
    let metadata_store = transaction.store("file_metadata")?;

    let metadata = metadata_store.get(&file_id.into()).await?;
    let metadata: Option<FileMetadata> = serde_wasm_bindgen::from_value(metadata).unwrap();

    Ok(metadata)
}

// get memory data
async fn get_memory_data(rexie: &Rexie, file_id: u32) -> Result<Option<MemoryData>> {
    let transaction = rexie.transaction(&["memory_data"], TransactionMode::ReadOnly)?;
    let memory_store = transaction.store("memory_data")?;

    let memory = memory_store.get(&file_id.into()).await?;
    let memory: Option<MemoryData> = serde_wasm_bindgen::from_value(memory).unwrap();

    Ok(memory)
}

// get a disassembly chunk
async fn get_disassembly_chunk(rexie: &Rexie, file_id: u32, chunk_id: u32) -> Result<Option<DisassemblyChunk>> {
    let transaction = rexie.transaction(&["disassembly_data"], TransactionMode::ReadOnly)?;
    let disassembly_store = transaction.store("disassembly_data")?;

    let id = serde_json::json!({ "file_id": file_id, "chunk_id": chunk_id });
    let id_js = serde_wasm_bindgen::to_value(&id).unwrap();

    let chunk = disassembly_store.get(&id_js).await?;
    let chunk: Option<DisassemblyChunk> = serde_wasm_bindgen::from_value(chunk).unwrap();

    Ok(chunk)
}
