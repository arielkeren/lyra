"use client";

import { createContext } from "react";
import { User } from "../types";

type UserContextType = {
  user: User | null | undefined;
  refreshUser: () => void;
  logout: () => void;
};

const UserContext = createContext<UserContextType | undefined>(undefined);

export default UserContext;
