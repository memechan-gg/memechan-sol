export const unsignedMultipartRequest = async (input: string, file: File) => {
  const formData = new FormData();

  formData.append("file", file);

  try {
    const r = await fetch(input, {
      method: "POST",
      headers: {
        Accept: file.type,
      },
      body: formData,
    });
    if (!r.ok) {
      const body = await r.json();
      throw new Error(JSON.stringify({ body, status: r.statusText }));
    }
    return r.json();
  } catch (error) {
    console.error("Upload failed:", error);
    throw error;
  }
};
