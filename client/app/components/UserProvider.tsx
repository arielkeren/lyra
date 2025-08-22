"use client";

import { useState, useCallback, useEffect } from "react";
import UserContext from "../contexts/UserContext";
import { isUser, User } from "../types";

type Props = {
  children: React.ReactNode;
};

const decodeJWT = (token: string): User | null => {
  try {
    const payload = token.split(".")[1];
    const decoded = JSON.parse(
      atob(payload.replace(/-/g, "+").replace(/_/g, "/"))
    );
    return isUser(decoded) ? decoded : null;
  } catch {
    return null;
  }
};

const UserProvider = ({ children }: Props) => {
  const [user, setUser] = useState<User | null | undefined>(undefined);

  const refreshUser = useCallback(() => {
    const token = localStorage.getItem("token");
    if (token) {
      const decoded = decodeJWT(token);
      setUser(decoded);
    } else setUser(null);
  }, []);

  const logout = useCallback(() => {
    localStorage.removeItem("token");
    setUser(null);
  }, []);

  useEffect(() => {
    refreshUser();
  }, [refreshUser]);

  return (
    <UserContext.Provider value={{ user, refreshUser, logout }}>
      {children}
    </UserContext.Provider>
  );
};

export default UserProvider;
