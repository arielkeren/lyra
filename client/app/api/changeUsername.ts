import { put } from "./methods";

const changeUsername = async (username: string, token: string) =>
  await put("users", token, {
    username,
    email: "",
    password: "",
  });

export default changeUsername;
