type Jsonable =
  | string
  | number
  | boolean
  | null
  | undefined
  | readonly Jsonable[]
  | { readonly [key: string]: Jsonable }
  | { toJSON(): Jsonable };

export class BaseError extends Error {
  public readonly context?: Jsonable;

  constructor(
    message: string,
    options: { error?: Error; context?: Jsonable } = {}
  ) {
    const { error: cause, context } = options;

    super(message, { cause });
    this.name = this.constructor.name;

    this.context = context;
  }
}

export type Result<T, E extends BaseError = BaseError> =
  | { success: true; result: T }
  | { success: false; error: E };
