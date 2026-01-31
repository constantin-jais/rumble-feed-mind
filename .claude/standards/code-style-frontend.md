# Code Style - Frontend

## Nommage

| Type | Convention | Exemple |
|------|------------|---------|
| Fichiers composants | PascalCase | `ToolCard.tsx` |
| Fichiers utils | kebab-case | `format-output.ts` |
| Composants | PascalCase | `ToolCard` |
| Hooks | camelCase + use | `useToolExecution` |
| Functions | camelCase | `formatOutput` |

## Composants

```typescript
// ✅ BON - Function declaration, props destructurées
export function ToolCard({ tool, onSelect }: ToolCardProps) {
  return <div>{tool.name}</div>;
}

// ❌ MAUVAIS - Arrow function
export const ToolCard = (props: ToolCardProps) => { ... };
```

## State Management

```typescript
// Server state → TanStack Query
const { data } = useQuery({ queryKey: ["tools"], queryFn: api.listTools });

// Global client state → Zustand
const { favorites } = useToolsStore();

// Local state → useState
const [input, setInput] = useState("");
```

## Styling

```typescript
// ✅ BON - CSS variables via Tailwind
<div className="bg-primary text-primary-foreground">

// ❌ MAUVAIS - Couleurs hardcodées
<div className="bg-blue-500">
```

## Types

```typescript
// Interface pour objets
interface Tool { id: string; name: string; }

// Type pour unions
type ToolCategory = "encoding" | "crypto";
```
