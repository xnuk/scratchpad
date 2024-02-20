import { date, number, string, tupleParser } from './mini-zod.ts'

const GOOGLE_API_KEY = 'MASKED'

type Range = `${string}!${string}:${string}`

const makeUrl = <R extends readonly Range[]>(
	spreadsheetId: string,
	ranges: R,
) =>
	new URL(
		// biome-ignore lint/style/useTemplate: I have more readability
		'https://sheets.googleapis.com/v4/spreadsheets/' +
			spreadsheetId +
			'/values:batchGet' +
			'?' +
			'dateTimeRenderOption=SERIAL_NUMBER' +
			'&' +
			'majorDimension=ROWS' +
			'&' +
			'valueRenderOption=UNFORMATTED_VALUE' +
			'&' +
			'prettyPrint=false' +
			'&' +
			ranges.map(v => `ranges=${encodeURIComponent(v)}`).join('&'),
	) as URL & {
		___responseType: BatchGetValues<R[number], 'ROWS'>
	}

type Values = readonly (readonly (string | number | boolean)[])[]

interface BatchGetValues<
	Ranges extends Range,
	Dimension extends 'ROWS' | 'COLS' = 'ROWS',
> {
	readonly spreadsheetId: string
	readonly valueRanges: readonly {
		readonly range: Ranges
		readonly majorDimension: Dimension
		readonly values: Values
	}[]
}

const hash = async (s: string) =>
	Array.from(
		new Uint8Array(
			await crypto.subtle.digest('SHA-256', new TextEncoder().encode(s)),
		),
	)
		.map(v => v.toString(16))
		.join('')

const request = async <T extends URL>(
	url: T,
): Promise<T extends { ___responseType: infer U } ? U : unknown> => {
	const resp = await fetch(url, {
		headers: {
			'x-goog-api-key': GOOGLE_API_KEY,
			accept: 'application/json; charset=utf-8',
		},
		redirect: 'error',
		method: 'GET',
	})

	const successful = resp.status === 200
	const contentType = resp.headers.get('content-type')
	const isJson = /^application\/json(?:;|$)/

	if (successful && contentType != null && isJson.test(contentType)) {
		return await resp.json()
	}

	return resp.text().then(Promise.reject)
}

// =and(iserr(IMPORTRANGE(F2,"Updating!B2")), not(iserr(IMPORTRANGE(F2,"CutS!B2"))))
// =transpose(importrange(Data!B1, "Spot!B16:AF21"))
const sheets = {
	라라펠: '1i9elmJmB53Y08v9tYhL5SkOc7hpyLNQdONv_E9OAvkQ',
	엘레젠: '1PGhy1KPKmhtxzHKgO0d5BKuIW2prtKQtAs9lbwC779o',
	루가딘: '1UvRY8V1tKv_EjfAL1vsmjt0aLuKqlz8AVYGHBVA4Y4c',
	미코테: '1P3byILrndBxD9Rpp7RE_KTK5dQfXVisHlVLXTrTWU7I',
	아우라: '1QCKOHzxcYBiMAy8i_JpxxXV1khCPhGFzeDANTZnqaII',
	비에라: '12aOEspfskJI53_LOBtcgjE1xSGxTztVnkMYvjeTiQ0s',
} as const

const fromEntriesByKey = <
	T extends { readonly [Key in K]: string },
	K extends keyof T,
>(
	arr: readonly (T | null | undefined)[],
	key: K,
): { readonly [Key in T[K]]: T } => {
	const res: { [Key in T[K]]: T } = Object.create(null)

	for (const item of arr) {
		if (item != null) {
			res[item[key]] = item
		}
	}

	return res
}

type TupleOf<T, N extends number> = N extends N
	? number extends N
		? T[]
		: _TupleOf<T, N, []>
	: never
type _TupleOf<T, N extends number, R extends unknown[]> = R['length'] extends N
	? R
	: _TupleOf<T, N, [T, ...R]>

const reqq = async <const T extends readonly Range[]>(
	spreadsheet: Extract<keyof typeof sheets, string>,
	ranges: T,
): Promise<
	T extends { length: infer N extends number } ? TupleOf<Values, N> : []
> => {
	const resp = await request(makeUrl(sheets[spreadsheet], ranges))
	return resp.valueRanges.map(v => v.values) as T extends {
		length: infer N extends number
	}
		? TupleOf<Values, N>
		: []
}

const [calt, cals, calins, calf, cuts, spot] = await reqq('라라펠', [
	'CalT!B9:E15',
	'CalS!AT4:BE47',
	'CalIns!AT30:BE70',
	'CalF!AT6:BE17',
	'CutS!B2:B2',
	'Spot!B16:AF21',
])

const trow = tupleParser([
	['server', string],
	['start', date],
	['end', date],
	['pivot', date],
])

const parseCalT = (values: Values) => {
	if ((values[0] || []).slice(1).join(',') !== '점검시작,점검종료,점검기준') {
		return null
	}

	return fromEntriesByKey(values.slice(1).map(trow), 'server')
}

const ttrow = tupleParser([
	['이름', string],
	['약칭', string],
	['지역', string],
	['최소', number],
	['최대', number],
	['점검최소', number],
	['점검최대', number],
	['모그리', date],
	['초코보', date],
	['카벙클', date],
	['톤베리', date],
	['펜리르', date],
])

const parseCal = (values: Values) => {
	if (
		values[0]?.join(',') !== ',,,,,,,모그리,초코보,카벙클,톤베리,펜리르' ||
		values[1]?.join(',') !==
			'이름,약칭,지역,최소,최대,점검최소,점검최대,컷타임,컷타임,컷타임,컷타임,컷타임'
	) {
		return null
	}

	return fromEntriesByKey(values.slice(2).map(ttrow), '이름')
}

const parseSpot = (values: Values) => {
	const checkAllString = (
		x: readonly unknown[] | null | undefined,
	): readonly string[] | null =>
		x?.every((v): v is string => typeof v === 'string') ? x : null

	const header = checkAllString(values[0])
	if (header == null || header[0] !== '자리 정리 현황') {
		return null
	}

	const res: { [key: string]: { [key: string]: string | null } } =
		Object.create(null)
	for (const item of values.slice(1)) {
		const row = checkAllString(item)
		if (row == null) {
			continue
		}
		const server = row[0]
		if (server == null) {
			continue
		}

		const obj: { [key: string]: string | null } = Object.create(null)
		for (let i = 1; i < header.length; ++i) {
			const key = header[i]
			const val = row[i]
			if (key == null || key === '' || val == null || val === '') {
				continue
			}
			obj[key] = val === '정리 필요' ? null : val
		}

		res[server] = obj
	}

	return res
}

console.log(
	JSON.stringify({
		time: parseCalT(calt),
		cals: parseCal(cals.concat(calins, calf)),
		spot: parseSpot(spot),
		cuts,
	}),
)
