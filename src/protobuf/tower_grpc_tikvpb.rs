use super::tikvpb::*;
pub mod client {
    use ::tower_grpc::codegen::client::*;
    use super::{BatchRaftMessage, BatchCommandsRequest, BatchCommandsResponse};

    /// Serve as a distributed kv database.
    #[derive(Debug, Clone)]
    pub struct Tikv<T> {
        inner: grpc::Grpc<T>,
    }

    impl<T> Tikv<T> {
        pub fn new(inner: T) -> Self {
            let inner = grpc::Grpc::new(inner);
            Self { inner }
        }

        /// Poll whether this client is ready to send another request.
        pub fn poll_ready<R>(&mut self) -> futures::Poll<(), grpc::Status>
        where T: grpc::GrpcService<R>,
        {
            self.inner.poll_ready()
        }

        /// Get a `Future` of when this client is ready to send another request.
        pub fn ready<R>(self) -> impl futures::Future<Item = Self, Error = grpc::Status>
        where T: grpc::GrpcService<R>,
        {
            futures::Future::map(self.inner.ready(), |inner| Self { inner })
        }

        /// Serve as a distributed kv database.
        pub fn snapshot<R, B>(&mut self, request: grpc::Request<B>) -> grpc::client_streaming::ResponseFuture<super::super::raft_serverpb::Done, T::Future, T::ResponseBody>
        where T: grpc::GrpcService<R>,
              B: futures::Stream<Item = super::super::raft_serverpb::SnapshotChunk>,
              B: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/tikvpb.Tikv/Snapshot");
            self.inner.client_streaming(request, path)
        }

        /// Serve as a distributed kv database.
        pub fn raft<R, B>(&mut self, request: grpc::Request<B>) -> grpc::client_streaming::ResponseFuture<super::super::raft_serverpb::Done, T::Future, T::ResponseBody>
        where T: grpc::GrpcService<R>,
              B: futures::Stream<Item = super::super::raft_serverpb::RaftMessage>,
              B: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/tikvpb.Tikv/Raft");
            self.inner.client_streaming(request, path)
        }

        /// Serve as a distributed kv database.
        pub fn batch_raft<R, B>(&mut self, request: grpc::Request<B>) -> grpc::client_streaming::ResponseFuture<super::super::raft_serverpb::Done, T::Future, T::ResponseBody>
        where T: grpc::GrpcService<R>,
              B: futures::Stream<Item = BatchRaftMessage>,
              B: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/tikvpb.Tikv/BatchRaft");
            self.inner.client_streaming(request, path)
        }

        /// Serve as a distributed kv database.
        pub fn batch_commands<R, B>(&mut self, request: grpc::Request<B>) -> grpc::streaming::ResponseFuture<BatchCommandsResponse, T::Future>
        where T: grpc::GrpcService<R>,
              B: futures::Stream<Item = BatchCommandsRequest>,
              B: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/tikvpb.Tikv/BatchCommands");
            self.inner.streaming(request, path)
        }
    }
}

pub mod server {
    use ::tower_grpc::codegen::server::*;
    use super::{BatchRaftMessage, BatchCommandsRequest, BatchCommandsResponse};

    // Redefine the try_ready macro so that it doesn't need to be explicitly
    // imported by the user of this generated code.
    macro_rules! try_ready {
        ($e:expr) => (match $e {
            Ok(futures::Async::Ready(t)) => t,
            Ok(futures::Async::NotReady) => return Ok(futures::Async::NotReady),
            Err(e) => return Err(From::from(e)),
        })
    }

    /// Serve as a distributed kv database.
    pub trait Tikv: Clone {
        type SnapshotFuture: futures::Future<Item = grpc::Response<super::super::raft_serverpb::Done>, Error = grpc::Status>;
        type RaftFuture: futures::Future<Item = grpc::Response<super::super::raft_serverpb::Done>, Error = grpc::Status>;
        type BatchRaftFuture: futures::Future<Item = grpc::Response<super::super::raft_serverpb::Done>, Error = grpc::Status>;
        type BatchCommandsStream: futures::Stream<Item = BatchCommandsResponse, Error = grpc::Status>;
        type BatchCommandsFuture: futures::Future<Item = grpc::Response<Self::BatchCommandsStream>, Error = grpc::Status>;

        fn snapshot(&mut self, request: grpc::Request<grpc::Streaming<super::super::raft_serverpb::SnapshotChunk>>) -> Self::SnapshotFuture;

        fn raft(&mut self, request: grpc::Request<grpc::Streaming<super::super::raft_serverpb::RaftMessage>>) -> Self::RaftFuture;

        fn batch_raft(&mut self, request: grpc::Request<grpc::Streaming<BatchRaftMessage>>) -> Self::BatchRaftFuture;

        fn batch_commands(&mut self, request: grpc::Request<grpc::Streaming<BatchCommandsRequest>>) -> Self::BatchCommandsFuture;
    }

    #[derive(Debug, Clone)]
    pub struct TikvServer<T> {
        tikv: T,
    }

    impl<T> TikvServer<T>
    where T: Tikv,
    {
        pub fn new(tikv: T) -> Self {
            Self { tikv }
        }
    }

    impl<T> tower::Service<http::Request<grpc::BoxBody>> for TikvServer<T>
    where T: Tikv,
    {
        type Response = http::Response<tikv::ResponseBody<T>>;
        type Error = grpc::Never;
        type Future = tikv::ResponseFuture<T>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(().into())
        }

        fn call(&mut self, request: http::Request<grpc::BoxBody>) -> Self::Future {
            use self::tikv::Kind::*;

            match request.uri().path() {
                "/tikvpb.Tikv/Snapshot" => {
                    let mut service = tikv::methods::Snapshot(self.tikv.clone());
                    let response = grpc::client_streaming(&mut service, request);
                    tikv::ResponseFuture { kind: Snapshot(response) }
                }
                "/tikvpb.Tikv/Raft" => {
                    let mut service = tikv::methods::Raft(self.tikv.clone());
                    let response = grpc::client_streaming(&mut service, request);
                    tikv::ResponseFuture { kind: Raft(response) }
                }
                "/tikvpb.Tikv/BatchRaft" => {
                    let mut service = tikv::methods::BatchRaft(self.tikv.clone());
                    let response = grpc::client_streaming(&mut service, request);
                    tikv::ResponseFuture { kind: BatchRaft(response) }
                }
                "/tikvpb.Tikv/BatchCommands" => {
                    let mut service = tikv::methods::BatchCommands(self.tikv.clone());
                    let response = grpc::streaming(&mut service, request);
                    tikv::ResponseFuture { kind: BatchCommands(response) }
                }
                _ => {
                    tikv::ResponseFuture { kind: __Generated__Unimplemented(grpc::unimplemented(format!("unknown service: {:?}", request.uri().path()))) }
                }
            }
        }
    }

    impl<T> tower::Service<()> for TikvServer<T>
    where T: Tikv,
    {
        type Response = Self;
        type Error = grpc::Never;
        type Future = futures::FutureResult<Self::Response, Self::Error>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(futures::Async::Ready(()))
        }

        fn call(&mut self, _target: ()) -> Self::Future {
            futures::ok(self.clone())
        }
    }

    impl<T> tower::Service<http::Request<tower_hyper::Body>> for TikvServer<T>
    where T: Tikv,
    {
        type Response = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Response;
        type Error = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Error;
        type Future = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Future;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            tower::Service::<http::Request<grpc::BoxBody>>::poll_ready(self)
        }

        fn call(&mut self, request: http::Request<tower_hyper::Body>) -> Self::Future {
            let request = request.map(|b| grpc::BoxBody::map_from(b));
            tower::Service::<http::Request<grpc::BoxBody>>::call(self, request)
        }
    }

    pub mod tikv {
        use ::tower_grpc::codegen::server::*;
        use super::Tikv;
        use super::super::{BatchRaftMessage, BatchCommandsRequest};

        pub struct ResponseFuture<T>
        where T: Tikv,
        {
            pub(super) kind: Kind<
                // Snapshot
                grpc::client_streaming::ResponseFuture<methods::Snapshot<T>, grpc::Streaming<super::super::super::raft_serverpb::SnapshotChunk>>,
                // Raft
                grpc::client_streaming::ResponseFuture<methods::Raft<T>, grpc::Streaming<super::super::super::raft_serverpb::RaftMessage>>,
                // BatchRaft
                grpc::client_streaming::ResponseFuture<methods::BatchRaft<T>, grpc::Streaming<BatchRaftMessage>>,
                // BatchCommands
                grpc::streaming::ResponseFuture<methods::BatchCommands<T>, grpc::Streaming<BatchCommandsRequest>>,
                // A generated catch-all for unimplemented service calls
                grpc::unimplemented::ResponseFuture,
            >,
        }

        impl<T> futures::Future for ResponseFuture<T>
        where T: Tikv,
        {
            type Item = http::Response<ResponseBody<T>>;
            type Error = grpc::Never;

            fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Snapshot(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: Snapshot(body) }
                        });
                        Ok(response.into())
                    }
                    Raft(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: Raft(body) }
                        });
                        Ok(response.into())
                    }
                    BatchRaft(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: BatchRaft(body) }
                        });
                        Ok(response.into())
                    }
                    BatchCommands(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: BatchCommands(body) }
                        });
                        Ok(response.into())
                    }
                    __Generated__Unimplemented(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: __Generated__Unimplemented(body) }
                        });
                        Ok(response.into())
                    }
                }
            }
        }

        pub struct ResponseBody<T>
        where T: Tikv,
        {
            pub(super) kind: Kind<
                // Snapshot
                grpc::Encode<grpc::unary::Once<<methods::Snapshot<T> as grpc::ClientStreamingService<grpc::Streaming<super::super::super::raft_serverpb::SnapshotChunk>>>::Response>>,
                // Raft
                grpc::Encode<grpc::unary::Once<<methods::Raft<T> as grpc::ClientStreamingService<grpc::Streaming<super::super::super::raft_serverpb::RaftMessage>>>::Response>>,
                // BatchRaft
                grpc::Encode<grpc::unary::Once<<methods::BatchRaft<T> as grpc::ClientStreamingService<grpc::Streaming<BatchRaftMessage>>>::Response>>,
                // BatchCommands
                grpc::Encode<<methods::BatchCommands<T> as grpc::StreamingService<grpc::Streaming<BatchCommandsRequest>>>::ResponseStream>,
                // A generated catch-all for unimplemented service calls
                (),
            >,
        }

        impl<T> tower::HttpBody for ResponseBody<T>
        where T: Tikv,
        {
            type Data = <grpc::BoxBody as grpc::Body>::Data;
            type Error = grpc::Status;

            fn is_end_stream(&self) -> bool {
                use self::Kind::*;

                match self.kind {
                    Snapshot(ref v) => v.is_end_stream(),
                    Raft(ref v) => v.is_end_stream(),
                    BatchRaft(ref v) => v.is_end_stream(),
                    BatchCommands(ref v) => v.is_end_stream(),
                    __Generated__Unimplemented(_) => true,
                }
            }

            fn poll_data(&mut self) -> futures::Poll<Option<Self::Data>, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Snapshot(ref mut v) => v.poll_data(),
                    Raft(ref mut v) => v.poll_data(),
                    BatchRaft(ref mut v) => v.poll_data(),
                    BatchCommands(ref mut v) => v.poll_data(),
                    __Generated__Unimplemented(_) => Ok(None.into()),
                }
            }

            fn poll_trailers(&mut self) -> futures::Poll<Option<http::HeaderMap>, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Snapshot(ref mut v) => v.poll_trailers(),
                    Raft(ref mut v) => v.poll_trailers(),
                    BatchRaft(ref mut v) => v.poll_trailers(),
                    BatchCommands(ref mut v) => v.poll_trailers(),
                    __Generated__Unimplemented(_) => Ok(None.into()),
                }
            }
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        pub(super) enum Kind<Snapshot, Raft, BatchRaft, BatchCommands, __Generated__Unimplemented> {
            Snapshot(Snapshot),
            Raft(Raft),
            BatchRaft(BatchRaft),
            BatchCommands(BatchCommands),
            __Generated__Unimplemented(__Generated__Unimplemented),
        }

        pub mod methods {
            use ::tower_grpc::codegen::server::*;
            use super::super::{Tikv, BatchRaftMessage, BatchCommandsRequest};

            pub struct Snapshot<T>(pub T);

            impl<T> tower::Service<grpc::Request<grpc::Streaming<super::super::super::super::raft_serverpb::SnapshotChunk>>> for Snapshot<T>
            where T: Tikv,
            {
                type Response = grpc::Response<super::super::super::super::raft_serverpb::Done>;
                type Error = grpc::Status;
                type Future = T::SnapshotFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<grpc::Streaming<super::super::super::super::raft_serverpb::SnapshotChunk>>) -> Self::Future {
                    self.0.snapshot(request)
                }
            }

            pub struct Raft<T>(pub T);

            impl<T> tower::Service<grpc::Request<grpc::Streaming<super::super::super::super::raft_serverpb::RaftMessage>>> for Raft<T>
            where T: Tikv,
            {
                type Response = grpc::Response<super::super::super::super::raft_serverpb::Done>;
                type Error = grpc::Status;
                type Future = T::RaftFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<grpc::Streaming<super::super::super::super::raft_serverpb::RaftMessage>>) -> Self::Future {
                    self.0.raft(request)
                }
            }

            pub struct BatchRaft<T>(pub T);

            impl<T> tower::Service<grpc::Request<grpc::Streaming<BatchRaftMessage>>> for BatchRaft<T>
            where T: Tikv,
            {
                type Response = grpc::Response<super::super::super::super::raft_serverpb::Done>;
                type Error = grpc::Status;
                type Future = T::BatchRaftFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<grpc::Streaming<BatchRaftMessage>>) -> Self::Future {
                    self.0.batch_raft(request)
                }
            }

            pub struct BatchCommands<T>(pub T);

            impl<T> tower::Service<grpc::Request<grpc::Streaming<BatchCommandsRequest>>> for BatchCommands<T>
            where T: Tikv,
            {
                type Response = grpc::Response<T::BatchCommandsStream>;
                type Error = grpc::Status;
                type Future = T::BatchCommandsFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<grpc::Streaming<BatchCommandsRequest>>) -> Self::Future {
                    self.0.batch_commands(request)
                }
            }
        }
    }
}
