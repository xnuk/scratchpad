const ASIA_SEOUL_SECONDS = 32400
const NOMATCH = Symbol('empty')

const toDate = (lotus: number) =>
	new Date((Math.round((lotus - 25569) * 86400) + ASIA_SEOUL_SECONDS) * 1000)

type Constraint<T> = (x: unknown) => T | typeof NOMATCH

export const number: Constraint<number> = x =>
	typeof x === 'number' ? x : NOMATCH

export const string: Constraint<Exclude<string, ''>> = x =>
	typeof x === 'string' && x.length > 0 ? x : NOMATCH

export const date: Constraint<Date> = x =>
	typeof x === 'number' ? toDate(x) : NOMATCH

export const nullable =
	<T>(c: Constraint<T>) =>
	(x: unknown): T | null => {
		const item = c(x)
		return item === NOMATCH ? null : item
	}

type ParsedRow<T extends readonly unknown[]> = T extends readonly [
	readonly [infer K extends string, Constraint<infer V>],
	...infer R,
]
	? {
			readonly [Key in K]: V
	  } & ParsedRow<R>
	: Record<never, never>

export const tupleParser =
	<const T extends readonly (readonly [string, Constraint<unknown>])[]>(
		cols: T,
	) =>
	(row: readonly unknown[]): ParsedRow<T> | null => {
		if (cols.length !== row.length) {
			return null
		}

		const ret: {
			[key: string]: unknown
		} = Object.create(null)

		for (let i = 0; i < cols.length; ++i) {
			const item = row[i]
			// biome-ignore lint/style/noNonNullAssertion: this is sane
			const [key, func] = cols[i]!
			const z = func(item)
			if (z === NOMATCH) {
				return null
			}
			ret[key] = z
		}

		return ret as ParsedRow<T>
	}
