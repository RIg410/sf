import { AuthService, AuthServiceClientImpl, GrpcWebImpl } from "@/generated/auth";
import { UsersService, UsersServiceClientImpl } from "@/generated/users";
import { getAuthService } from "./auth";
import { grpc } from "@improbable-eng/grpc-web";

export const GRPC_URL = "http://localhost:3000";

export class gRPC {
    authService: AuthService;
    userService: UsersService;

    constructor(grpcClient: GrpcWebImpl) {
        this.authService = new AuthServiceClientImpl(grpcClient);
        this.userService = new UsersServiceClientImpl(grpcClient);

    }
}


let clients = createGRPCClient();

export function getGRPC(): gRPC {
    return clients;
}

function createGRPCClient(): gRPC {
    const auth = getAuthService();
    const token = auth.getToken();

    const metadata = new grpc.Metadata();
    metadata.set("Authorization", token ? `Bearer ${token}` : "");
    const client = new GrpcWebImpl(GRPC_URL, {
        metadata,
    });
    return new gRPC(client);
}

export function initGRPC() {
    clients = createGRPCClient();
}