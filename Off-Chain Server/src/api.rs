// api.rs

#[get("/stream/<stream_id>")]
fn get_stream_data(stream_id: u64) -> Json<PaymentStream> {
    // Fetch and return stream data from the database
}
