
struct AssoCache<T>
{
	/** Number of ways */
	ways: u64;
	/** Number of rows */
	rows: u64;
	/** Store content **/
	content: Vec<T>,
	/** store addresses identifying content to match requests **/
	addr: Vec<usize>,
	/** Used to round robin on rows to override **/
	next: u8,
	/** count miss **/
	miss: u64,
	/** counter hits **/
	hits: u64,
	/** counter flush */
	flush_count: u64,
	/** Has been flushed recently, not do do again */
	diry: bool
}

impl AssoCache<T>
{
	pub fn new<T>(ways: u64, rows: u64) -> AssoCache<T>
	{
		AssoCache<T>{
			content: vec![T::default(); ways * rows];
		}
	}
}

// template<class T,size_t ways,size_t rows>
// class StaticAssoCache
// {
// 	public:
// 		StaticAssoCache(void);
// 		void flush(void);
// 		const T * get(size_t addr) const;
// 		void set(size_t addr,const T & value);
// 		void unset(size_t addr);
// 		void printStats(const char * name) const;
// 	private:
// 		/** Store content **/
// 		T content[rows][ways];
// 		/** store addresses identifying content to match requests **/
// 		size_t addr[rows][ways];
// 		/** Used to round robin on rows to override **/
// 		unsigned char next[rows];
// 		/** count miss **/
// 		mutable size_t miss{0};
// 		/** counter hits **/
// 		mutable size_t hits{0};
// 		/** counter flush */
// 		mutable size_t flushCnt{0};
// 		/** Has been flushed recently, not do do again */
// 		bool dirty{true};
// };