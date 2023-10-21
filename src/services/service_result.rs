pub enum ServiceResult<M> {
    Response(M),
    Continue,
    Break,
}
