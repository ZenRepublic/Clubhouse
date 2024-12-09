#!/usr/bin/env npx ts-node

import * as fs from 'fs/promises';
import * as path from 'path';

const INPUT_PATH = './target/idl/clubhouse.json';
const OUTPUT_DIR = './target/idl-sdk';
const OUTPUT_PATH = path.join(OUTPUT_DIR, 'clubhouse.json');

type JsonData = {
  types?: Array<{ name?: string; [key: string]: any }>;
  accounts?: Array<{ name?: string; [key: string]: any }>;
  [key: string]: any;
};

async function main() {
  try {
    const content = await fs.readFile(INPUT_PATH, 'utf-8');
    const data: JsonData = JSON.parse(content);
    const transformed = transformJson(data);
    await fs.mkdir(OUTPUT_DIR, { recursive: true });
    await fs.writeFile(OUTPUT_PATH, JSON.stringify(transformed, null, 2));
    console.log('Successfully transformed IDL');
  } catch (error) {
    console.error('Error:', error);
    process.exit(1);
  }
}

function transformJson(data: JsonData): JsonData {
  const transformed = JSON.parse(JSON.stringify(data));
  return mergeTypesIntoAccounts(
    simplifyTypeDefinitions(
      replacePubkey(transformed)
    )
  );
}

function replacePubkey(obj: any): any {
  if (Array.isArray(obj)) {
    return obj.map(replacePubkey);
  }
  if (obj && typeof obj === 'object') {
    const newObj = { ...obj };
    
    if (newObj.type === 'pubkey') {
      newObj.type = 'publicKey';
    }

    if (newObj.type?.option === 'pubkey') {
        newObj.type.option = 'publicKey';
      }
    
    // Recursively process all values
    for (const key in newObj) {
      newObj[key] = replacePubkey(newObj[key]);
    }
    
    return newObj;
  }
  return obj;
}

function simplifyTypeDefinitions(obj: any): any {
    if (Array.isArray(obj)) {
      return obj.map(simplifyTypeDefinitions);
    }
    if (obj && typeof obj === 'object') {
      const newObj = { ...obj };
      
      // Recursively check for 'defined' pattern at any nesting level
      for (const key in newObj) {
        if (newObj[key] && typeof newObj[key] === 'object') {
          if (newObj[key].defined?.name) {
            newObj[key].defined = newObj[key].defined.name;
          }
          newObj[key] = simplifyTypeDefinitions(newObj[key]);
        }
      }
      
      return newObj;
    }
    return obj;
  }

function mergeTypesIntoAccounts(data: JsonData): JsonData {
  if (!data.types || !data.accounts) return data;

  const typeProps = data.types.reduce((acc, type) => {
    if (type.name) {
      const { name, ...props } = type;
      acc[name] = props;
    }
    return acc;
  }, {} as Record<string, any>);

  data.accounts = data.accounts.map(account => {
    if (account.name && typeProps[account.name]) {
      return { ...account, ...typeProps[account.name] };
    }
    return account;
  });

  return data;
}

main().catch(console.error);