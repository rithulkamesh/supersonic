import type { NextApiRequest, NextApiResponse } from "next";
import mysql from "mysql";

// const db = mysql.createConnection(process.env.DATABASE_URL || "");
const Config = (req: NextApiRequest, res: NextApiResponse) => {
  res.status(200).json({ res: "ok" });
};

export default Config;
