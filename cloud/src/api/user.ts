import { ProductEdition } from "types/Template";
import ajax from "utils/ajax";

const user = () => ajax("/user");

export interface UserDTO {
  email: string;
  passwd: string;
  validationCode: string;
}

export interface AuthUserDTO {
  name: string;
  email: string;
  edition: ProductEdition;
  token: string;
}

export function registerValidationCode(email: string) {
  return user().path("/register-validate-code").payload({ email }).post();
}

export function resetPasswdValidationCode(email: string) {
  return user().path("/reset-validate-code").payload({ email }).post();
}

export function register(dto: UserDTO) {
  return user().payload(dto).post();
}

export function resetPasswd(dto: UserDTO) {
  return user().path("/passwd").payload(dto).post();
}

export function login(email: string, passwd: string): Promise<AuthUserDTO> {
  return ajax("/token")
    .payload({ email, passwd })
    .post() as Promise<AuthUserDTO>;
}

export function currentUser() {
  return user().get() as Promise<AuthUserDTO>;
}

export type AuthUser = AuthUserDTO | null;

export async function setAuthUser(token: AuthUser) {
  localStorage.setItem("authUser", JSON.stringify(token));
}

export async function getAuthUser() {
  const value = localStorage.getItem("authUser");
  return value ? (JSON.parse(value) as AuthUser) : null;
}
