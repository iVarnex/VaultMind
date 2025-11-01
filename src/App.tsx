import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";

function App() {
  // Theme management
  useEffect(() => {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    document.documentElement.classList.toggle('dark', prefersDark);
  }, []);

  // Crypto test state
  const [plaintext, setPlaintext] = useState("Hello, World!");
  const [password, setPassword] = useState("supersecret");
  const [ciphertext, setCiphertext] = useState("");
  const [decryptedText, setDecryptedText] = useState("");
  const [error, setError] = useState("");

  // DB test state
  const [itemName, setItemName] = useState("My First Note");
  const [itemContent, setItemContent] = useState("This is the content of the first note.");
  const [dbStatus, setDbStatus] = useState("");

  const handleEncrypt = async () => {
    setError("");
    setCiphertext("");
    setDecryptedText("");
    try {
      const result = await invoke<string>("encrypt", { data: plaintext, password });
      setCiphertext(result);
    } catch (e) {
      setError(e as string);
    }
  };

  const handleDecrypt = async () => {
    setError("");
    setDecryptedText("");
    if (!ciphertext) {
      setError("Ciphertext is empty. Please encrypt first.");
      return;
    }
    try {
      const result = await invoke<string>("decrypt", { encodedData: ciphertext, password });
      setDecryptedText(result);
    } catch (e) {
      setError(e as string);
    }
  };

  const handleAddItem = async () => {
    setDbStatus("");
    try {
      await invoke("add_item", { name: itemName, content: itemContent });
      setDbStatus(`Successfully added item: '${itemName}'`);
    } catch (e) {
      setDbStatus(`Error: ${e as string}`);
    }
  };

  return (
    <main className="container flex flex-col items-center justify-start min-h-screen bg-background text-foreground p-8">
      <div className="w-full max-w-2xl">
        <h1 className="text-3xl font-bold mb-6 text-center">VaultMind Crypto Test</h1>
        
        <div className="flex flex-col gap-4">
          {/* Plaintext Input */}
          <div>
            <label htmlFor="plaintext" className="block text-sm font-medium mb-1">Plaintext</label>
            <Textarea
              id="plaintext"
              value={plaintext}
              onChange={(e) => setPlaintext(e.target.value)}
              placeholder="Enter text to encrypt"
            />
          </div>

          {/* Password Input */}
          <div>
            <label htmlFor="password" className="block text-sm font-medium mb-1">Password</label>
            <Input
              id="password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="Enter password"
            />
          </div>

          {/* Action Buttons */}
          <div className="flex gap-4">
            <Button onClick={handleEncrypt} className="w-full">Encrypt</Button>
            <Button onClick={handleDecrypt} variant="secondary" className="w-full">Decrypt</Button>
          </div>

          {/* Results */}
          <div className="mt-4 space-y-4">
            {ciphertext && (
              <div>
                <h2 className="font-semibold">Ciphertext (Base64):</h2>
                <p className="text-sm text-muted-foreground break-all bg-muted p-2 rounded-md">{ciphertext}</p>
              </div>
            )}
            {decryptedText && (
              <div>
                <h2 className="font-semibold">Decrypted Text:</h2>
                <p className="text-sm text-green-600 bg-green-100 dark:bg-green-900 p-2 rounded-md">{decryptedText}</p>
              </div>
            )}
            {error && (
              <div>
                <h2 className="font-semibold">Error:</h2>
                <p className="text-sm text-destructive bg-destructive/20 p-2 rounded-md">{error}</p>
              </div>
            )}
          </div>
        </div>

        <hr className="my-8 border-border" />

        {/* DB Test Section */}
        <div className="flex flex-col gap-4">
            <h1 className="text-3xl font-bold mb-2 text-center">DB Test</h1>
            <div>
                <label htmlFor="itemName" className="block text-sm font-medium mb-1">Item Name</label>
                <Input id="itemName" value={itemName} onChange={(e) => setItemName(e.target.value)} />
            </div>
            <div>
                <label htmlFor="itemContent" className="block text-sm font-medium mb-1">Item Content</label>
                <Textarea id="itemContent" value={itemContent} onChange={(e) => setItemContent(e.target.value)} />
            </div>
            <Button onClick={handleAddItem}>Add Item to DB</Button>
            {dbStatus && (
                <p className={`text-sm p-2 rounded-md ${dbStatus.startsWith("Error") ? "text-destructive bg-destructive/20" : "text-green-600 bg-green-100 dark:bg-green-900"}`}>
                    {dbStatus}
                </p>
            )}
        </div>

      </div>
    </main>
  );
}

export default App;
