"use client";

import getUser from "@/app/api/getUser";
import Header from "@/app/components/Header";
import PackageNameList from "@/app/components/PackageNameList";
import Render from "@/app/components/Render";
import useFetch from "@/app/hooks/useFetch";
import { isString } from "@/app/types";
import { useParams } from "next/navigation";

const UserPage = () => {
  const params = useParams();
  const user = useFetch(() => getUser(isString(params.id) ? params.id : ""));

  return (
    <>
      <Header />
      <main className="min-h-screen flex flex-col items-center justify-center text-white px-4">
        <Render
          data={user}
          loading={<div>Loading...</div>}
          error={<div>Error loading user</div>}
          content={user => (
            <div className="text-center space-y-8 w-full max-w-3xl">
              <p>{user.username}</p>
              <p>
                Joined:{" "}
                {new Date(user.createdAt).toLocaleDateString("en-US", {
                  year: "numeric",
                  month: "long",
                  day: "numeric",
                })}
              </p>
              <div className="flex justify-end w-full mb-4">
                <PackageNameList packages={user.packagesCreated} />
              </div>
            </div>
          )}
        />
      </main>
    </>
  );
};

export default UserPage;
