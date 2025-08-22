import { isOtherUser } from "../types";
import { get } from "./methods";

const getUser = async (id: string) => {
  const data = await get(`users/${id}`);
  return isOtherUser(data) ? data : null;
};

export default getUser;
