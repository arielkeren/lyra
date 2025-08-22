import Link from "next/link";

const Home = () => (
  <main className="h-screen flex flex-col items-center justify-center text-white px-4">
    <div className="text-center space-y-8">
      <h1 className="text-9xl font-extrabold tracking-tight drop-shadow-lg">
        Lyra
      </h1>
      <p className="text-2xl md:text-3xl font-light max-w-3xl mx-auto drop-shadow-lg">
        A modern programming language embracing{" "}
        <span className="font-bold text-fuchsia-400 drop-shadow-md">
          simplicity
        </span>
        {", "}
        <span className="font-bold text-indigo-300 drop-shadow-md">
          clarity
        </span>
        {" and "}
        <span className="font-bold text-purple-300 drop-shadow-md">speed</span>
      </p>
      <div className="flex flex-col md:flex-row gap-6 justify-center mt-8">
        <Link
          href="/docs"
          className="text-2xl bg-gradient-to-r from-fuchsia-500 via-pink-500 to-purple-500 hover:from-fuchsia-600 hover:to-purple-600 text-white font-extrabold py-4 px-10 rounded-xl shadow-2xl transform hover:scale-105 transition-all duration-200 border-2 border-fuchsia-300 relative"
        >
          🚀 <span className="relative z-10 drop-shadow-lg">Get Started</span>
        </Link>
        <Link
          href="/packages"
          className="text-2xl bg-gradient-to-r from-indigo-500 via-purple-500 to-fuchsia-500 hover:from-indigo-600 hover:to-fuchsia-600 text-white font-extrabold py-4 px-10 rounded-xl shadow-2xl transform hover:scale-105 transition-all duration-200 border-2 border-indigo-300 relative"
        >
          📦{" "}
          <span className="relative z-10 drop-shadow-lg">Browse Packages</span>
        </Link>
      </div>
    </div>
  </main>
);

export default Home;
