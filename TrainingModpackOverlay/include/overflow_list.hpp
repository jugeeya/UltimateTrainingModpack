#include <tesla.hpp>

class OverflowList : public tsl::elm::List {
	public:
	OverflowList() : tsl::elm::List() {}

	virtual tsl::elm::Element* requestFocus(tsl::elm::Element *oldFocus, tsl::FocusDirection direction) override {
		tsl::elm::Element *newFocus = nullptr;

		if (this->m_clearList || this->m_itemsToAdd.size() > 0)
			return nullptr;

		if (direction == tsl::FocusDirection::None) {
			u16 i = 0;

			if (oldFocus == nullptr) {
				s32 elementHeight = 0;
				while (elementHeight < this->m_offset && i < this->m_items.size() - 1) {
					i++;
					elementHeight += this->m_items[i]->getHeight();
				}
			}

			for (; i < this->m_items.size(); i++) {
				newFocus = this->m_items[i]->requestFocus(oldFocus, direction);

				if (newFocus != nullptr) {
					this->m_focusedIndex = i;

					this->updateScrollOffset();
					return newFocus;
				}
			}
		} else {
			if (direction == tsl::FocusDirection::Down) {

				for (u16 i = this->m_focusedIndex + 1; i < this->m_items.size(); i++) {
					newFocus = this->m_items[i]->requestFocus(oldFocus, direction);

					if (newFocus != nullptr && newFocus != oldFocus) {
						this->m_focusedIndex = i;

						this->updateScrollOffset();
						return newFocus;
					}
				}

				if (this->m_focusedIndex == this->m_items.size() - 1) {
                    for (u16 i = 0; i < this->m_items.size(); i++) {
                        newFocus = this->m_items[i]->requestFocus(oldFocus, direction);

                        if (newFocus != nullptr && newFocus != oldFocus) {
                            this->m_focusedIndex = i;

                            this->updateScrollOffset();
                            return newFocus;
                        }
                    }
				}

				return oldFocus;
			} else if (direction == tsl::FocusDirection::Up) {
				if (this->m_focusedIndex > 0) {

					for (u16 i = this->m_focusedIndex - 1; i >= 0; i--) {
                        if (i > this->m_items.size() || this->m_items[i] == nullptr)
                            return oldFocus;
                        else
						    newFocus = this->m_items[i]->requestFocus(oldFocus, direction);

						if (newFocus != nullptr && newFocus != oldFocus) {
							this->m_focusedIndex = i;

							this->updateScrollOffset();
							return newFocus;
						}
					}

				} 
                
                for (u16 i = this->m_items.size() - 1; i >= 0; i--) {
                    if (i <= this->m_items.size() && this->m_items[i] != nullptr)
                        newFocus = this->m_items[i]->requestFocus(oldFocus, direction);

                    if (newFocus != nullptr && newFocus != oldFocus) {
                        this->m_focusedIndex = i;

                        this->updateScrollOffset();
                        return newFocus;
                    }
                }

				return oldFocus;
			}
		}

		return oldFocus;
	}

	private:
		virtual void updateScrollOffset() {
			if (this->getInputMode() != tsl::InputMode::Controller)
				return;

			if (this->m_listHeight <= this->getHeight()) {
				this->m_nextOffset = 0;
				this->m_offset = 0;

				return;
			}

			this->m_nextOffset = 0;
			for (u16 i = 0; i < this->m_focusedIndex; i++)
				this->m_nextOffset += this->m_items[i]->getHeight();

			this->m_nextOffset -= this->getHeight() / 3;

			if (this->m_nextOffset < 0)
				this->m_nextOffset = 0;

			if (this->m_nextOffset > (this->m_listHeight - this->getHeight()) + 50)
				this->m_nextOffset = (this->m_listHeight - this->getHeight() + 50);
		}
};