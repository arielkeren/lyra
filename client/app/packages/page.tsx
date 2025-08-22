"use client";

import getPackages from "../api/getPackages";
import useFetch from "../hooks/useFetch";
import PackageList from "../components/PackageList";
import Render from "../components/Render";
import Header from "../components/Header";

const PackagesPage = () => {
  const packages = useFetch(getPackages);

  return (
    <>
      <Header />

      <main className="min-h-screen flex flex-col items-center justify-center text-white px-4">
        <div className="text-center space-y-8 w-full max-w-3xl">
          <h1 className="text-6xl md:text-7xl font-extrabold tracking-tight drop-shadow-lg mb-4">
            📦 Lyra Packages
          </h1>
          <p className="text-xl md:text-2xl font-light mb-8 drop-shadow-lg">
            Discover and share packages
          </p>

          <Render
            data={packages}
            loading={
              <div className="text-2xl animate-pulse">Loading packages...</div>
            }
            error={
              <div className="text-red-400 text-xl">
                Failed to fetch packages
              </div>
            }
            content={packages => <PackageList packages={packages} />}
          />
        </div>
      </main>
    </>
  );
};

export default PackagesPage;
