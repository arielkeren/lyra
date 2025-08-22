import { put } from "./methods";

const changeEmail = async (email: string, token: string) =>
  await put("users", token, {
    email,
    username: "",
    password: "",
  });

export default changeEmail;
