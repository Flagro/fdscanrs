import fdscanrs.fdscanrs as rust_fdscan

class FDScanRS:
    """
    A Python wrapper for the Rust-based FDScanRS library to mine candidate keys
    from datasets efficiently using concurrency.
    """

    @staticmethod
    def mine_candidate_keys(data):
        """
        Mine candidate keys from the given dataset using the Rust library.

        Args:
            data (list of list of str): The dataset to analyze, represented as a list of rows,
                                        where each row is a list of column values.

        Returns:
            list of list of int: A list of candidate keys, where each candidate key is represented
                                 as a list of column indices.
        """
        if not isinstance(data, list) or not all(isinstance(row, list) for row in data):
            raise ValueError("Input data must be a list of lists (e.g., a 2D array).")

        # Call the Rust function directly
        return rust_fdscan.mine_candidate_keys(data)
